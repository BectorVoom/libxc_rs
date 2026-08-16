//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3187/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3187<F: Float>(t15503: F, t15640: F, t19025: F, t3535: F, t1202: F, t19032: F, t15498: F, t4993: F, t15486: F, t5024: F, t1090: F, t11668: F, t11678: F, t1218: F, t1227: F, t1232: F, t15654: F, t15708: F, t18205: F, t18941: F, t3243: F, t3447: F, t3494: F, t3577: F, t3578: F, t45128: F, t4582: F, t4729: F, t4987: F, t5012: F, t52935: F, t52942: F, t53249: F, t55716: F, t5971: F, t61798: F, t61910: F, t6225: F) -> F {
    let t66120 = t15503 * t15640;
    let t66147 = t3535 * t19025;
    let t66150 = t1202 * t19032;
    let t66153 = t15498 * t4993;
    let t66155 = t5024 * t15486;
    let t66157 = -t3577 * t3578 * t5012 * t4729 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t3577 * t45128 * t18205 * t15708 - t66120 / F::cast_from(108.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1227 * t4582 * t4987 * t61798 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1227 * t4582 * t15654 * t61910 - t52935 / F::cast_from(432.0_f64) - F::cast_from(7.0_f64) / F::cast_from(162.0_f64) * t3447 * t53249 * t55716 - t3577 * t3578 * t18941 * t1090 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t11678 * t11668 * t6225 * t3243 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3577 * t11668 * t5971 * t3494 - t52942 / F::cast_from(1728.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t66147 * t1218 - F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t66150 * t1232 + t66153 / F::cast_from(324.0_f64) + t66155 / F::cast_from(324.0_f64);
    t66157
}
