//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 612/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk612(t24: f64, t1651: f64, t3371: f64, t3374: f64, t91: f64, t3370: f64, t98: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t3378 = piecewise3(t90, 0.0_f64, 4.0_f64 / 9.0_f64 * t1651 * t3371 + 4.0_f64 / 3.0_f64 * t91 * t3374);
    let t3380 = (t3370 + t3378) * t98;
    t3380
}
