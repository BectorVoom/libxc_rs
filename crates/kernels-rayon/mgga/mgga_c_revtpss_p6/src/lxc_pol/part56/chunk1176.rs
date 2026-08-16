//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1176/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1176(t1243: f64, t1802: f64, t8944: f64, t8938: f64, t246: f64, t3598: f64, t33518: f64, t5265: f64, t33502: f64, t5362: f64, t104504: f64, t1122: f64, t124569: f64, t124584: f64, t124644: f64, t124646: f64, t124650: f64, t1248: f64, t1250: f64, t13126: f64, t1769: f64, t2148: f64, t33425: f64, t33525: f64, t34964: f64, t3626: f64, t5279: f64, t5287: f64, t5334: f64, t5351: f64, t5353: f64) -> (f64, f64, f64, f64) {
    let t131394 = t1243 * t1802;
    let t131395 = t8944 * t131394;
    let t131396 = t8938 * t131395;
    let t131416 = t3598 * t246;
    let t131421 = t33518 * t5265;
    let t131423 = t33502 * t5362;
    let t131426 = 0.18822977838986977999e-3_f64 * t33425 * t3626 * t34964 * t1122 - 0.66110807482757352569e-3_f64 * t131396 * t33525 - 0.3718732920905101082e-3_f64 * t33518 * t5287 - 0.3718732920905101082e-3_f64 * t124584 * t5279 + 0.56468933516960933998e-3_f64 * t124650 * t124646 * t5351 * t104504 - 0.56468933516960933998e-3_f64 * t124644 * t124646 * t5353 + 0.56468933516960933998e-3_f64 * t124650 * t124646 * t1769 * t1248 * t1250 - 0.28234466758480466999e-3_f64 * t2148 * t13126 * t131416 * t124646 * t5334 - 0.24791552806034007213e-3_f64 * t131421 + 0.3718732920905101082e-3_f64 * t131423 - 0.37187329209051010821e-3_f64 * t124569;
    (t131394, t131395, t131416, t131426)
}
