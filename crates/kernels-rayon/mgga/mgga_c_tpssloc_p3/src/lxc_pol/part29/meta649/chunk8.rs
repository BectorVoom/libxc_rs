//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2166/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2166(t12971: f64, t13196: f64, t1484: f64, t1877: f64, t1915: f64, t23286: f64, t23290: f64, t23295: f64, t2522: f64, t25354: f64, t25358: f64, t2745: f64, t2749: f64, t4255: f64, t4303: f64, t4314: f64, t47645: f64, t57921: f64, t58009: f64, t58071: f64, t59580: f64, t6666: f64, t6670: f64, t7634: f64, t776: f64, t86706: f64, t86713: f64, t86815: f64, t87975: f64) -> f64 {
    let t89822 = 3.0_f64 * t12971 * t1915 * t2522 + 6.0_f64 * t13196 * t1915 * t4314 + 3.0_f64 * t1484 * t23286 * t2522 - 2.0_f64 * t1877 * t23290 * t4303 + 4.0_f64 * t1877 * t23295 * t58009 + 2.0_f64 * t1877 * t23295 * t86713 - t1877 * t25358 * t2745 + 2.0_f64 * t1877 * t2749 * t87975 + 6.0_f64 * t23295 * t2522 * t57921 + 6.0_f64 * t2522 * t25354 * t776 - 6.0_f64 * t2522 * t58071 * t6670 - 3.0_f64 * t2522 * t59580 * t6670 - 3.0_f64 * t2522 * t6670 * t86815 + 12.0_f64 * t4255 * t4314 * t6666 - 6.0_f64 * t4314 * t6670 * t86706 + 6.0_f64 * t47645 * t7634;
    t89822
}
