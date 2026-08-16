//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3765/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765(t1250: f64, t1261: f64, t1264: f64, t12800: f64, t16729: f64, t17344: f64, t17351: f64, t17369: f64, t17467: f64, t17514: f64, t17693: f64, t20945: f64, t21153: f64, t247: f64, t3647: f64, t3718: f64, t3719: f64, t3720: f64, t44521: f64, t5052: f64, t5333: f64, t5373: f64, t5391: f64, t58909: f64, t6679: f64, t68391: f64, t71061: f64, t71827: f64, t71839: f64, t71845: f64, t71854: f64, t71859: f64) -> f64 {
    let t71867 = -0.19055119163586549765e-3_f64 * t71827 + 0.15244095330869239812e-2_f64 * t5391 * t17369 - 0.14291339372689912324e-3_f64 * t12800 * t6679 - 0.28582678745379824648e-3_f64 * t3647 * t21153 - 0.14291339372689912324e-3_f64 * t1261 * t247 * t1264 * t68391 - 0.12862205435420921092e-2_f64 * t17344 * t247 * t3719 * t71839 + 0.11433071498151929859e-2_f64 * t71845 + 0.11433071498151929859e-2_f64 * t17351 * t58909 * t5333 * t5052 - 0.11433071498151929859e-2_f64 * t44521 * t71061 * t17514 - 0.42874018118069736972e-3_f64 * t3718 * t3720 * t71854 * t1250 + 0.30488190661738479624e-2_f64 * t71859 + 0.47637797908966374413e-3_f64 * t17693 * t20945 * t1250 * t16729 - 2.0_f64 / 81.0_f64 * t5373 * t17467;
    t71867
}
