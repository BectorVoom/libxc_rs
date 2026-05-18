//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1008/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1008<F: Float>(t1286: F, t137311: F, t144623: F, t144633: F, t144635: F, t144641: F, t144643: F, t144645: F, t1564: F, t25533: F, t25570: F, t25574: F, t25579: F, t28: F, t3051: F, t32016: F, t32355: F, t34575: F, t497: F, t5501: F, t7161: F, t925: F) -> F {
    let t144647 = t1286 * t28 * t34575 * t497 / F::new(6.0) - t1286 * t28 * t32355 * t25533 / F::new(3.0) + t144623 / F::new(54.0) - t32016 * t25574 / F::new(18.0) - t7161 * t3051 * t25579 / F::new(9.0) - t32016 * t25570 / F::new(18.0) - t144633 / F::new(9.0) - t144635 / F::new(18.0) - t5501 * t1564 * t137311 * t925 / F::new(18.0) + t144641 / F::new(9.0) - F::new(4.0) * t144643 - F::new(2.0) * t144645;
    t144647
}
