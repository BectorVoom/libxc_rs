//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1397/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1397(t1004: f64, t1015: f64, t10410: f64, t10415: f64, t10857: f64, t23419: f64, t23457: f64, t23483: f64, t23495: f64, t23504: f64, t23515: f64, t23521: f64, t23548: f64, t23556: f64, t23564: f64, t25652: f64, t25654: f64, t25660: f64, t3073: f64, t3120: f64, t3128: f64, t3131: f64, t360: f64, t378: f64, t6723: f64, t6730: f64, t6735: f64, t6742: f64, t6744: f64, t68: f64, t82911: f64, t82987: f64, t82990: f64, t83117: f64, t83172: f64, t83196: f64, t83206: f64, t83215: f64, t83220: f64) -> f64 {
    let t83223 = 19.0_f64 / 432.0_f64 * t83172 + 19.0_f64 / 288.0_f64 * t1004 * t23556 * t378 - 0.30279567070605293142e-3_f64 * t23564 * t23504 + 0.60559134141210586284e-3_f64 * t25652 * t3128 * t3120 * t25654 - 0.30279567070605293142e-3_f64 * t25652 * t1015 * t3120 * t25660 - 0.60559134141210586284e-3_f64 * t82911 * t23515 + 0.48447307312968469026e-2_f64 * t23457 * t6735 - 0.30279567070605293142e-3_f64 * t6730 * t23548 + 0.24223653656484234513e-2_f64 * t6723 * t23495 - 0.60559134141210586284e-3_f64 * t82987 * t83196 * t82990 * t3131 - 0.30279567070605293142e-3_f64 * t83117 * t23521 - 0.24223653656484234513e-2_f64 * t23483 * t23504 + 0.30279567070605293142e-3_f64 * t83206 + 0.10093189023535097714e-3_f64 * t6742 * t6744 * t10857 * t68 * t360 + 5.0_f64 / 2304.0_f64 * t23419 * t10410 - t83215 * t10415 / 768.0_f64 - t83220 * t3073 / 72.0_f64;
    t83223
}
