//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2646/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2646(t25: f64, t15937: f64, t15940: f64, t16557: f64, t19547: f64, t20216: f64, t20376: f64, t2219: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t606: f64, t67059: f64, t73975: f64, t73978: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t73989 = piecewise3(t26, 0.0_f64, 40.0_f64 / 81.0_f64 * t39419 * t20376 * t606 - 16.0_f64 / 9.0_f64 * t19547 * t2219 - 8.0_f64 / 9.0_f64 * t15937 * t73975 + 8.0_f64 / 3.0_f64 * t15940 * t73978 + 4.0_f64 / 3.0_f64 * t5134 * t16557 + 4.0_f64 / 9.0_f64 * t3664 * t20216 * t606 + 4.0_f64 / 3.0_f64 * t514 * t67059);
    t73989
}
