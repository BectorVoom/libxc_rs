//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1283/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1283(t1090: f64, t117809: f64, t117934: f64, t118038: f64, t1238: f64, t1241: f64, t125295: f64, t125306: f64, t125311: f64, t125313: f64, t125358: f64, t125383: f64, t125530: f64, t125568: f64, t1720: f64, t2128: f64, t24567: f64, t24589: f64, t24601: f64, t27445: f64, t27786: f64, t27830: f64, t32451: f64, t32489: f64, t32493: f64, t32514: f64, t32538: f64, t34250: f64, t3598: f64, t4733: f64, t4940: f64, t4945: f64, t498: f64, t5055: f64, t5088: f64, t7283: f64, t7287: f64, t7392: f64, t8002: f64, t8882: f64, t8897: f64) -> f64 {
    let t125580 = 0.54831135561607547883e-2_f64 * t24589 * t118038 * t8002 - 0.10966227112321509577e-1_f64 * t24589 * t117809 * t27445 + 0.18277045187202515961e-2_f64 * t117934 + 0.54831135561607547883e-2_f64 * t24589 * t24601 * t32514 * t4733 - 0.9869604401089358619e-1_f64 * t2128 * t24601 * t27786 + 0.54831135561607547883e-2_f64 * t24589 * t24601 * t125295 * t1090 + 2.0_f64 * t1238 * t3598 * t8897 * t5088 - 2.0_f64 * t27830 * t7392 - 0.54831135561607547883e-2_f64 * t7283 * t125306 * t7287 + 0.54831135561607547883e-2_f64 * t125311 - 0.54831135561607547883e-2_f64 * t125313 + 4.0_f64 * t5055 * t32493 - 6.0_f64 * t5055 * t32538 + 2.0_f64 * t4945 * t32489 - t1238 * t1241 * (t125358 + t125383 + t125530 + t125568) + t4940 * t8882 * t498 + t1720 * t32451 * t498 - 0.16449340668482264365e-1_f64 * t7283 * t24567 * t34250;
    t125580
}
