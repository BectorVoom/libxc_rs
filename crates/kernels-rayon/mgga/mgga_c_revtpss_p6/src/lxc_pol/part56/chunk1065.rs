//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1065/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1065(t121346: f64, t119971: f64, t32237: f64, t121142: f64, t1412: f64, t844: f64, t32291: f64, t8591: f64, t121166: f64, t25304: f64, t8571: f64, t121035: f64, t32268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121347 = 0.18822977838986977999e-5_f64 * t121346;
    let t121348 = t119971 * t32237;
    let t121350 = 0.6019057092162847523e-2_f64 * t121348 * t121142;
    let t121354 = t844 * t1412;
    let t121356 = t8591 * t121354 * t32291;
    let t121363 = t25304 * t8571 * t121166;
    let t121364 = 0.17851433602423232928e-4_f64 * t121363;
    let t121365 = t32268 * t121035;
    (t121347, t121350, t121354, t121356, t121364, t121365)
}
