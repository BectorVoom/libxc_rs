//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1156/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1156<F: Float>(t3766: F, t8214: F, t3033: F, t9838: F, t22722: F, t3769: F, t10013: F, t11181: F, t1209: F, t18427: F, t18750: F, t18863: F, t22230: F, t22639: F, t22706: F, t22745: F, t27262: F, t27295: F, t27694: F, t3083: F, t3103: F, t31067: F, t31088: F, t3116: F, t3136: F, t365: F, t3792: F, t3793: F, t3820: F, t3823: F, t6288: F, t8102: F, t8115: F, t8153: F, t870: F, t9891: F, t9930: F, t9959: F, t9964: F) -> (F, F, F, F) {
    let t31186 = 3.0 * t8214 * t3766;
    let t31188 = 3.0 * t3033 * t9838;
    let t31190 = 0.48245938496077605201e2 * t22722 * t3769;
    let t31191 = -0.310907e-1 * (t18750 - 0.53272592592592592592e-1 * t18427 - 0.15981777777777777777e0 * t22230 + t22706 + 0.68493333333333333332e-1 * t27295 - 0.51369999999999999999e-1 * t27262 - 0.17123333333333333333e-1 * t31067 + 0.5137e-1 * t31088) * t365 + 0.62071215503128080361e4 * t6288 * t3792 * t8153 * t870 - 0.57895126195293126241e3 * t22639 * t10013 + 3.0 * t9891 * t3103 + 3.0 * t8115 * t3793 + 3.0 * t3083 * t9959 + 0.17544670867903938621e1 * t27694 * t1209 + 0.17544670867903938621e1 * t9964 * t3136 + 0.17544670867903938621e1 * t8102 * t3820 + 0.17544670867903938621e1 * t3116 * t9930 + 0.51947577317044391276e2 * t22745 * t3823 - 0.10389515463408878255e3 * t18863 * t11181 - t31186 - t31188 - t31190;
    (t31186, t31188, t31190, t31191)
}
