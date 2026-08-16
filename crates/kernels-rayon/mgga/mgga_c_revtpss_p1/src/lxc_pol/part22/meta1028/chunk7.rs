//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3611/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3611(t68415: f64, t68429: f64, t68443: f64, t68461: f64, t1132: f64, t56187: f64, t56189: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64, t56228: f64, t56230: f64, t56236: f64, t68389: f64, t68393: f64, t68397: f64, t68399: f64, t68402: f64) -> (f64, f64, f64) {
    let t68463 = t68415 + t68429 + t68443 + t68461;
    let t68464 = t1132 * t68463;
    let t68466 = -0.40256666666666666668e0_f64 * t56187 - 0.12077e1_f64 * t56189 + 0.26837777777777777778e0_f64 * t56209 + 0.13418888888888888889e0_f64 * t56212 + 0.80513333333333333335e0_f64 * t56214 - 0.22364814814814814815e0_f64 * t56216 + 0.53675555555555555558e0_f64 * t56228 - 0.20128333333333333334e0_f64 * t56230 - 0.62621481481481481484e0_f64 * t56236 - 0.20128333333333333334e0_f64 * t68389 + 0.301925e0_f64 * t68393 - 0.40256666666666666666e0_f64 * t68397 + 0.26837777777777777777e0_f64 * t68399 + 0.36793333333333333333e-1_f64 * t68402 + 0.258925e1_f64 * t68464;
    (t68463, t68464, t68466)
}
