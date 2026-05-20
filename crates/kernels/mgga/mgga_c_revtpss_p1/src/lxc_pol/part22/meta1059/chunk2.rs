//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3765/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3765<F: Float>(t1250: F, t1261: F, t1264: F, t12800: F, t16729: F, t17344: F, t17351: F, t17369: F, t17467: F, t17514: F, t17693: F, t20945: F, t21153: F, t247: F, t3647: F, t3718: F, t3719: F, t3720: F, t44521: F, t5052: F, t5333: F, t5373: F, t5391: F, t58909: F, t6679: F, t68391: F, t71061: F, t71827: F, t71839: F, t71845: F, t71854: F, t71859: F) -> F {
    let t71867 = -F::cast_from(0.19055119163586549765e-3_f64) * t71827 + F::cast_from(0.15244095330869239812e-2_f64) * t5391 * t17369 - F::cast_from(0.14291339372689912324e-3_f64) * t12800 * t6679 - F::cast_from(0.28582678745379824648e-3_f64) * t3647 * t21153 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t247 * t1264 * t68391 - F::cast_from(0.12862205435420921092e-2_f64) * t17344 * t247 * t3719 * t71839 + F::cast_from(0.11433071498151929859e-2_f64) * t71845 + F::cast_from(0.11433071498151929859e-2_f64) * t17351 * t58909 * t5333 * t5052 - F::cast_from(0.11433071498151929859e-2_f64) * t44521 * t71061 * t17514 - F::cast_from(0.42874018118069736972e-3_f64) * t3718 * t3720 * t71854 * t1250 + F::cast_from(0.30488190661738479624e-2_f64) * t71859 + F::cast_from(0.47637797908966374413e-3_f64) * t17693 * t20945 * t1250 * t16729 - F::new(2.0) / F::new(81.0) * t5373 * t17467;
    t71867
}
