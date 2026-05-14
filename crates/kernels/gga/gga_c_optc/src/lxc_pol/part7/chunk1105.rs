//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1105/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1105<F: Float>(t2583: F, t7879: F, t3843: F, t898: F, t893: F, t2649: F, t7878: F, t22021: F, t2648: F, t894: F, t2596: F, t7882: F, t25020: F, t25022: F, t25025: F, t25030: F, t25033: F, t25036: F, t25041: F, t25044: F, t2591: F, t2623: F, t2650: F, t7862: F, t7872: F, t7886: F, t7891: F) -> (F, F, F, F, F) {
    let t25050 = t2583 * t7879;
    let t25052 = t3843 * t898;
    let t25053 = t893 * t25052;
    let t25055 = t7878 * t2649;
    let t25056 = t893 * t25055;
    let t25059 = t894 * t2648 * t22021;
    let t25063 = t894 * t2596 * t22021;
    let t25068 = t2583 * t7882;
    let t25070 = -0.77272546575900069819e-1 * t25020 - 0.12878757762650011637e0 * t25022 + 0.42074449172244793097e-1 * t25025 + 0.3863627328795003491e0 * t2583 * t7872 - 0.24147670804968771818e-2 * t25030 - 0.40246118008281286364e-2 * t25033 - 0.48295341609937543636e-1 * t25036 + 0.36704459623552533164e0 * t7886 * t2591 - t25041 / 36.0 + t25044 / 54.0 + 2.0 / 9.0 * t2623 * t7862 - 4.0 / 27.0 * t2623 * t7891 + 0.25757515525300023273e-1 * t25050 + 0.21464596271083352727e-2 * t25053 + 0.48295341609937543636e-2 * t25056 - 0.10866451862235947318e-1 * t893 * t25059 + 0.90553765518632894319e-2 * t893 * t25063 - 0.73408919247105066328e0 * t7886 * t2650 + 0.15454509315180013964e0 * t25068;
    (t25052, t25055, t25059, t25063, t25070)
}
