//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1214/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1214<F: Float>(t25055: F, t893: F, t22021: F, t2648: F, t894: F, t2596: F, t2583: F, t7882: F, t25020: F, t25022: F, t25025: F, t25030: F, t25033: F, t25036: F, t25041: F, t25044: F, t25050: F, t25053: F, t2591: F, t2623: F, t2650: F, t7862: F, t7872: F, t7886: F, t7891: F) -> (F, F, F) {
    let t25056 = t893 * t25055;
    let t25059 = t894 * t2648 * t22021;
    let t25063 = t894 * t2596 * t22021;
    let t25068 = t2583 * t7882;
    let t25070 = -F::cast_from(0.77272546575900069819e-1_f64) * t25020 - F::cast_from(0.12878757762650011637e0_f64) * t25022 + F::cast_from(0.42074449172244793097e-1_f64) * t25025 + F::cast_from(0.3863627328795003491e0_f64) * t2583 * t7872 - F::cast_from(0.24147670804968771818e-2_f64) * t25030 - F::cast_from(0.40246118008281286364e-2_f64) * t25033 - F::cast_from(0.48295341609937543636e-1_f64) * t25036 + F::cast_from(0.36704459623552533164e0_f64) * t7886 * t2591 - t25041 / F::cast_from(36.0_f64) + t25044 / F::cast_from(54.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2623 * t7862 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t2623 * t7891 + F::cast_from(0.25757515525300023273e-1_f64) * t25050 + F::cast_from(0.21464596271083352727e-2_f64) * t25053 + F::cast_from(0.48295341609937543636e-2_f64) * t25056 - F::cast_from(0.10866451862235947318e-1_f64) * t893 * t25059 + F::cast_from(0.90553765518632894319e-2_f64) * t893 * t25063 - F::cast_from(0.73408919247105066328e0_f64) * t7886 * t2650 + F::cast_from(0.15454509315180013964e0_f64) * t25068;
    (t25059, t25063, t25070)
}
