//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2539/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2539<F: Float>(t52037: F, t52126: F, t3011: F, t4682: F, t11506: F, t1626: F, t1609: F, t2924: F, t51973: F, t52035: F, t2942: F, t4644: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t52598 = F::cast_from(0.45908888888888888888e0_f64) * t52037;
    let t52623 = F::cast_from(0.34731666666666666667e0_f64) * t52126;
    let t52637 = t4682 * t3011;
    let t52642 = t1626 * t11506;
    let t52645 = t2924 * t1609;
    let t52701 = F::cast_from(0.39862222222222222223e0_f64) * t51973;
    let t52751 = F::cast_from(0.27385555555555555556e0_f64) * t52126;
    let t52774 = F::cast_from(0.23744444444444444444e-1_f64) * t51973;
    let t52783 = F::cast_from(0.47488888888888888888e-1_f64) * t52035;
    let t52784 = F::cast_from(0.15829629629629629629e-1_f64) * t52037;
    let t52809 = t4644 * t2942;
    (t52598, t52623, t52637, t52642, t52645, t52701, t52751, t52774, t52783, t52784, t52809)
}
