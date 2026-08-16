//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1273/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1273(t24574: f64, t34310: f64, t34247: f64, t34323: f64, t32496: f64, t7999: f64, t34349: f64, t11605: f64, t11606: f64, t117897: f64, t117910: f64, t117924: f64, t1238: f64, t2121: f64, t225: f64, t24589: f64, t24880: f64, t27406: f64, t27721: f64, t27761: f64, t27784: f64, t32523: f64, t32538: f64, t32544: f64, t32547: f64, t34314: f64, t3593: f64, t462: f64, t4945: f64, t497: f64, t5059: f64, t5088: f64, t7351: f64, t8088: f64, t8887: f64, t8897: f64, t94458: f64) -> f64 {
    let t125254 = t24574 * t34310;
    let t125266 = t24574 * t34247;
    let t125270 = t24574 * t34323;
    let t125276 = t7999 * t32496;
    let t125278 = t24574 * t34349;
    let t125280 = -2.0_f64 * t24880 * t8088 + 4.0_f64 * t3593 * t34314 - 0.54831135561607547883e-2_f64 * t117897 + 0.54831135561607547883e-2_f64 * t24589 * t94458 * t32523 - 6.0_f64 * t1238 * t11606 * t8887 * t5088 - 0.18277045187202515961e-2_f64 * t117910 - 6.0_f64 * t4945 * t32538 - 0.18277045187202515961e-2_f64 * t125254 + 0.43864908449286038307e-1_f64 * t27406 * t32544 + 0.43864908449286038307e-1_f64 * t27406 * t32547 + 0.16449340668482264365e-1_f64 * t2121 * t462 * t27721 * t225 * t497 - 0.54831135561607547883e-2_f64 * t117924 - 0.54831135561607547883e-2_f64 * t125266 + 4.0_f64 * t7351 * t27761 + 0.10966227112321509577e-1_f64 * t125270 - 6.0_f64 * t27784 * t11605 * t8897 * t5059 - 0.14621636149762012769e-1_f64 * t125276 + 0.54831135561607547883e-2_f64 * t125278;
    t125280
}
