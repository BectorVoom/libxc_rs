//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1005/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1005(t12234: f64, t3516: f64, t1196: f64, t1130: f64, t3376: f64, t1151: f64, t3379: f64, t3428: f64, t1126: f64, t3432: f64, t3436: f64, t3431: f64, t418: f64) -> (f64, f64, f64, f64, f64) {
    let t12235 = t12234 * t3516;
    let t12237 = 0.35089341735807877242e1_f64 * t1196 * t12235;
    let t12238 = t3376 * t1130;
    let t12240 = 3.0_f64 * t12238 * t1151;
    let t12242 = 3.0_f64 * t3379 * t3428;
    let t12243 = t1126 * t3432;
    let t12245 = 0.48245938496077605201e2_f64 * t12243 * t3436;
    let t12247 = 1.0_f64 / t3431 / t418;
    (t12237, t12240, t12242, t12245, t12247)
}
