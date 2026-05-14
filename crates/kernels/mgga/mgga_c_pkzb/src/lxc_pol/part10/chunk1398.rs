//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1398/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1398<F: Float>(t2412: F, t3186: F, t179: F, t2405: F, t404: F, t9795: F, t1238: F, t8245: F, t3730: F, t6380: F, t8397: F, t2395: F, t3876: F, t5939: F, t11476: F, t23207: F, t23215: F, t23248: F, t23250: F, t23278: F, t2363: F, t2371: F, t2393: F, t25324: F, t3026: F, t3061: F, t3174: F, t3185: F, t3207: F, t402: F, t6518: F, t8254: F, t8259: F, t8344: F, t8349: F, t8428: F) -> (F,) {
    let t28089 = t2412 * t3186;
    let t28111 = t404 * t179 * t2405 * t9795;
    let t28113 = t1238 * t8245;
    let t28121 = t404 * t179 * t6380 * t3730;
    let t28123 = t1238 * t8397;
    let t28128 = t2395 * t5939 * t3876;
    let t28130 = 0.17149607247227894789e-2 * t23207 - 0.51448821741683684366e-2 * t2393 * t402 * t25324 * t28089 * t3207 * t3061 + 0.10289764348336736873e-1 * t2363 * t402 * t25324 * t28089 * t11476 * t8259 - 0.17149607247227894789e-2 * t3185 * t8254 * t2371 * t8344 - 0.22866142996303859718e-2 * t23215 - 0.51448821741683684367e-2 * t8428 * t8254 * t6518 * t8349 - 0.57165357490759649296e-3 * t28111 + 0.60976381323476959248e-2 * t28113 - t3174 * t23278 * t3061 * t3026 / 4.0 + 0.95275595817932748827e-4 * t28121 - 0.10162730220579493208e-2 * t28123 - 0.15244095330869239812e-2 * t23248 - 0.30488190661738479624e-2 * t23250 + 0.47637797908966374413e-4 * t28128;
    (t28130,)
}
