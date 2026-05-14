//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 884/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk884<F: Float>(t136015: F, t6418: F, t1286: F, t34588: F, t376: F, t25545: F, t7162: F, t25587: F, t26061: F, t5743: F, t137561: F, t979: F, t137311: F, t1564: F, t25533: F, t25570: F, t25574: F, t25579: F, t28: F, t3051: F, t32016: F, t32355: F, t34575: F, t497: F, t5501: F, t7161: F, t925: F) -> (F, F, F) {
    let t144623 = t136015 * t6418;
    let t144633 = t1286 * t376 * t34588;
    let t144635 = t7162 * t25545;
    let t144641 = t7162 * t25587;
    let t144643 = t26061 * t5743;
    let t144645 = t137561 * t979;
    let t144647 = t1286 * t28 * t34575 * t497 / 6.0 - t1286 * t28 * t32355 * t25533 / 3.0 + t144623 / 54.0 - t32016 * t25574 / 18.0 - t7161 * t3051 * t25579 / 9.0 - t32016 * t25570 / 18.0 - t144633 / 9.0 - t144635 / 18.0 - t5501 * t1564 * t137311 * t925 / 18.0 + t144641 / 9.0 - 4.0 * t144643 - 2.0 * t144645;
    (t144643, t144645, t144647)
}
