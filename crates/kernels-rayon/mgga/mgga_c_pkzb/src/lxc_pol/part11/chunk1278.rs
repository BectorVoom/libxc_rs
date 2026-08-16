//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1278/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1278(t3766: f64, t8214: f64, t3033: f64, t9838: f64, t22722: f64, t3769: f64, t10013: f64, t11181: f64, t1209: f64, t18427: f64, t18750: f64, t18863: f64, t22230: f64, t22639: f64, t22706: f64, t22745: f64, t27262: f64, t27295: f64, t27694: f64, t3083: f64, t3103: f64, t31067: f64, t31088: f64, t3116: f64, t3136: f64, t365: f64, t3792: f64, t3793: f64, t3820: f64, t3823: f64, t6288: f64, t8102: f64, t8115: f64, t8153: f64, t870: f64, t9891: f64, t9930: f64, t9959: f64, t9964: f64) -> (f64, f64, f64, f64) {
    let t31186 = 3.0_f64 * t8214 * t3766;
    let t31188 = 3.0_f64 * t3033 * t9838;
    let t31190 = 0.48245938496077605201e2_f64 * t22722 * t3769;
    let t31191 = -0.310907e-1_f64 * (t18750 - 0.53272592592592592592e-1_f64 * t18427 - 0.15981777777777777777e0_f64 * t22230 + t22706 + 0.68493333333333333332e-1_f64 * t27295 - 0.51369999999999999999e-1_f64 * t27262 - 0.17123333333333333333e-1_f64 * t31067 + 0.5137e-1_f64 * t31088) * t365 + 0.62071215503128080361e4_f64 * t6288 * t3792 * t8153 * t870 - 0.57895126195293126241e3_f64 * t22639 * t10013 + 3.0_f64 * t9891 * t3103 + 3.0_f64 * t8115 * t3793 + 3.0_f64 * t3083 * t9959 + 0.17544670867903938621e1_f64 * t27694 * t1209 + 0.17544670867903938621e1_f64 * t9964 * t3136 + 0.17544670867903938621e1_f64 * t8102 * t3820 + 0.17544670867903938621e1_f64 * t3116 * t9930 + 0.51947577317044391276e2_f64 * t22745 * t3823 - 0.10389515463408878255e3_f64 * t18863 * t11181 - t31186 - t31188 - t31190;
    (t31186, t31188, t31190, t31191)
}
