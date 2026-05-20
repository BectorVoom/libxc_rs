//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1012/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1012<F: Float>(t1774: F, t494: F, t247: F, t3719: F, t8931: F, t33462: F, t1287: F, t1791: F, t1797: F, t33398: F, t33405: F, t33423: F, t33425: F, t33456: F, t33461: F, t33469: F, t33477: F, t33495: F, t33502: F, t33509: F, t33518: F, t33523: F, t34901: F, t34905: F, t34909: F, t34915: F, t34920: F, t34925: F, t34931: F) -> (F, F, F, F) {
    let t34934 = t494 * t1774;
    let t34936 = t247 * t3719 * t34934;
    let t34939 = t8931 * t1774;
    let t34940 = t33462 * t34939;
    let t34943 = -t33523 + F::cast_from(0.3718732920905101082e-3_f64) * t33509 * t34901 - F::cast_from(0.18822977838986977999e-3_f64) * t33425 * t34905 + F::cast_from(0.11423947533020470523e1_f64) * t33477 * t34909 - F::cast_from(0.3718732920905101082e-3_f64) * t33518 * t1797 + F::cast_from(0.17135921299530705785e1_f64) * t33461 * t34915 - F::cast_from(0.5578099381357651623e-3_f64) * t33495 * t34920 + F::cast_from(0.5578099381357651623e-3_f64) * t33502 * t1791 + t33423 - F::cast_from(0.8673628188205199462e0_f64) * t33456 * t34925 * t1287 + F::cast_from(0.56468933516960933998e-3_f64) * t33398 * t34931 - F::cast_from(0.56468933516960933998e-3_f64) * t33405 * t34936 - F::cast_from(0.17135921299530705785e1_f64) * t33469 * t34940;
    (t34934, t34939, t34940, t34943)
}
