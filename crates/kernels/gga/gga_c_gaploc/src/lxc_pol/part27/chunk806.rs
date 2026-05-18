//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 806/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk806<F: Float>(t2021: F, t7634: F, t2052: F, t954: F, t1880: F, t2581: F, t1445: F, t2572: F, t4614: F, t7132: F, t4752: F, t740: F) -> (F, F, F, F, F, F) {
    let t7635 = t2021 * t7634;
    let t7638 = t2052 * t954;
    let t7643 = t2581 * t1880;
    let t7644 = t1445 * t7643;
    let t7647 = t4614 * t2572;
    let t7650 = t1445 * t7132;
    let t7653 = t4752 * t740;
    (t7635, t7638, t7644, t7647, t7650, t7653)
}
