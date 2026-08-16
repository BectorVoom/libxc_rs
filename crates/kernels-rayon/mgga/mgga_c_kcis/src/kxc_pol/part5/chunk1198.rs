//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1198/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1198(t238: f64, t6676: f64, t86: f64, t3393: f64, t6661: f64, t6665: f64, t1154: f64, t167: f64, t5153: f64, t10541: f64, t15008: f64, t19972: f64, t19974: f64, t19977: f64, t19980: f64, t19983: f64, t19986: f64, t19989: f64, t19992: f64, t19996: f64, t20000: f64, t20003: f64, t2429: f64, t368: f64, t5133: f64) -> f64 {
    let t20008 = t86 * t238 * t6676;
    let t20010 = t3393 * t6661;
    let t20012 = t3393 * t6665;
    let t20015 = t1154 * t5153 * t167;
    let t20018 = -t10541 + 0.35374814814814814815e-1_f64 * t19972 - 0.15918666666666666667e0_f64 * t5133 * t19974 + 0.26531111111111111111e0_f64 * t5133 * t19977 - 0.11791604938271604938e0_f64 * t5133 * t19980 - 0.17687407407407407407e0_f64 * t15008 * t19983 + 0.21224888888888888889e0_f64 * t15008 * t19986 + 0.53062222222222222222e-1_f64 * t5133 * t19989 - 0.44218518518518518518e-1_f64 * t5133 * t19992 + 0.10612444444444444444e0_f64 * t5133 * t19996 - 0.88437037037037037037e-1_f64 * t5133 * t20000 - 0.39796666666666666666e-1_f64 * t86 * t368 * t20003 - 0.26531111111111111111e-1_f64 * t20008 - 0.29479012345679012345e-1_f64 * t20010 - 0.35374814814814814815e-1_f64 * t20012 - 0.10612444444444444444e0_f64 * t2429 * t20015;
    t20018
}
