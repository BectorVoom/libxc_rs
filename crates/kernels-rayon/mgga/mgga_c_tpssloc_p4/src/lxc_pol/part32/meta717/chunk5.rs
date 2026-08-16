//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2276/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2276(t1530: f64, t16662: f64, t17109: f64, t1877: f64, t1915: f64, t23290: f64, t23295: f64, t2522: f64, t25358: f64, t25374: f64, t28448: f64, t28732: f64, t4119: f64, t4303: f64, t4314: f64, t46341: f64, t5527: f64, t5660: f64, t5664: f64, t6666: f64, t6670: f64, t67123: f64, t67164: f64, t7541: f64, t776: f64, t81539: f64, t868: f64, t86836: f64, t87975: f64, t98030: f64, t98054: f64, t98102: f64) -> f64 {
    let t100623 = 4.0_f64 * t1877 * t87975 * t25374 - 6.0_f64 * t2522 * t6670 * t67164 + 2.0_f64 * t1877 * t23295 * t98102 - t1877 * t98054 * t868 + 6.0_f64 * t46341 * t28732 + 4.0_f64 * t1877 * t23295 * t98030 - t1877 * t6670 * t17109 + 2.0_f64 * t1877 * t81539 * t5664 + 6.0_f64 * t2522 * t7541 * t4119 - 3.0_f64 * t2522 * t6670 * t67123 + 3.0_f64 * t2522 * t1915 * t16662 - 2.0_f64 * t1877 * t86836 * t1530 - 2.0_f64 * t1877 * t25358 * t4303 + 6.0_f64 * t4314 * t6666 * t5527 - t1877 * t23290 * t5660 + 3.0_f64 * t2522 * t28448 * t776;
    t100623
}
