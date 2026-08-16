//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1214/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1214(t25055: f64, t893: f64, t22021: f64, t2648: f64, t894: f64, t2596: f64, t2583: f64, t7882: f64, t25020: f64, t25022: f64, t25025: f64, t25030: f64, t25033: f64, t25036: f64, t25041: f64, t25044: f64, t25050: f64, t25053: f64, t2591: f64, t2623: f64, t2650: f64, t7862: f64, t7872: f64, t7886: f64, t7891: f64) -> (f64, f64, f64) {
    let t25056 = t893 * t25055;
    let t25059 = t894 * t2648 * t22021;
    let t25063 = t894 * t2596 * t22021;
    let t25068 = t2583 * t7882;
    let t25070 = -0.77272546575900069819e-1_f64 * t25020 - 0.12878757762650011637e0_f64 * t25022 + 0.42074449172244793097e-1_f64 * t25025 + 0.3863627328795003491e0_f64 * t2583 * t7872 - 0.24147670804968771818e-2_f64 * t25030 - 0.40246118008281286364e-2_f64 * t25033 - 0.48295341609937543636e-1_f64 * t25036 + 0.36704459623552533164e0_f64 * t7886 * t2591 - t25041 / 36.0_f64 + t25044 / 54.0_f64 + 2.0_f64 / 9.0_f64 * t2623 * t7862 - 4.0_f64 / 27.0_f64 * t2623 * t7891 + 0.25757515525300023273e-1_f64 * t25050 + 0.21464596271083352727e-2_f64 * t25053 + 0.48295341609937543636e-2_f64 * t25056 - 0.10866451862235947318e-1_f64 * t893 * t25059 + 0.90553765518632894319e-2_f64 * t893 * t25063 - 0.73408919247105066328e0_f64 * t7886 * t2650 + 0.15454509315180013964e0_f64 * t25068;
    (t25059, t25063, t25070)
}
