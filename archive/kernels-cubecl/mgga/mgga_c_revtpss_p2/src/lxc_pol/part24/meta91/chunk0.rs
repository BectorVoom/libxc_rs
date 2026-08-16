//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 531/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk531<F: Float>(t2470: F, t874: F, t875: F, t251: F, t2718: F, t1941: F, t268: F, t271: F, t1065: F, t159: F) -> (F, F, F, F, F) {
    let t2810 = F::cast_from(0.13009920719177044025e-1_f64) * t874 * t875 * t2470;
    let t2811 = t2718 * t251;
    let t2846 = t268 * t1941 * t271;
    let t2847 = F::cast_from(0.23744444444444444444e-1_f64) * t2846;
    let t2850 = t159 * t1065;
    (t2810, t2811, t2846, t2847, t2850)
}
