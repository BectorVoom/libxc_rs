//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1012/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1012(t1774: f64, t494: f64, t247: f64, t3719: f64, t8931: f64, t33462: f64, t1287: f64, t1791: f64, t1797: f64, t33398: f64, t33405: f64, t33423: f64, t33425: f64, t33456: f64, t33461: f64, t33469: f64, t33477: f64, t33495: f64, t33502: f64, t33509: f64, t33518: f64, t33523: f64, t34901: f64, t34905: f64, t34909: f64, t34915: f64, t34920: f64, t34925: f64, t34931: f64) -> (f64, f64, f64, f64) {
    let t34934 = t494 * t1774;
    let t34936 = t247 * t3719 * t34934;
    let t34939 = t8931 * t1774;
    let t34940 = t33462 * t34939;
    let t34943 = -t33523 + 0.3718732920905101082e-3_f64 * t33509 * t34901 - 0.18822977838986977999e-3_f64 * t33425 * t34905 + 0.11423947533020470523e1_f64 * t33477 * t34909 - 0.3718732920905101082e-3_f64 * t33518 * t1797 + 0.17135921299530705785e1_f64 * t33461 * t34915 - 0.5578099381357651623e-3_f64 * t33495 * t34920 + 0.5578099381357651623e-3_f64 * t33502 * t1791 + t33423 - 0.8673628188205199462e0_f64 * t33456 * t34925 * t1287 + 0.56468933516960933998e-3_f64 * t33398 * t34931 - 0.56468933516960933998e-3_f64 * t33405 * t34936 - 0.17135921299530705785e1_f64 * t33469 * t34940;
    (t34934, t34939, t34940, t34943)
}
