//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 873/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk873(t16065: f64, t5425: f64, t1889: f64, t3761: f64, t3809: f64, t1319: f64, t5477: f64, t10269: f64, t1102: f64, t16379: f64, t16384: f64, t16390: f64, t16393: f64, t16398: f64, t16401: f64, t16404: f64, t16408: f64, t16410: f64, t16413: f64, t16418: f64, t16423: f64, t16427: f64, t16432: f64, t16436: f64, t16438: f64, t16441: f64, t4587: f64) -> f64 {
    let t16443 = t5425 * t16065;
    let t16447 = t3761 * t1889 * t3809;
    let t16451 = t3761 * t5477 * t1319;
    let t16454 = -0.65704296666666666667e-3_f64 * t1102 * t16379 - 0.10950716111111111111e-2_f64 * t1102 * t16384 + 0.59133867e-2_f64 * t1102 * t16390 + 0.39422577999999999999e-2_f64 * t1102 * t16393 + 0.13140859333333333333e-2_f64 * t1102 * t16398 + 0.21901432222222222222e-3_f64 * t16401 - 0.59133867e-2_f64 * t1102 * t16404 - t16408 + t16410 - 0.36958666875e-3_f64 * t1102 * t16413 + 0.295669335e-2_f64 * t1102 * t16418 - 0.295669335e-2_f64 * t1102 * t16423 - 0.1478346675e-2_f64 * t1102 * t16427 - 0.19711289e-2_f64 * t1102 * t16432 + t16436 - 0.13140859333333333333e-2_f64 * t10269 * t16438 - 0.32852148333333333333e-3_f64 * t16441 - 0.43802864444444444444e-2_f64 * t4587 * t16443 + 0.98556445e-3_f64 * t1102 * t16447 - 0.39422578e-2_f64 * t4587 * t16451;
    t16454
}
