//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1312/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1312(t10277: f64, t976: f64, t1021: f64, t10263: f64, t10403: f64, t1041: f64, t10493: f64, t248: f64, t2776: f64, t3039: f64, t3048: f64, t3070: f64, t3071: f64, t3121: f64, t3132: f64, t3146: f64, t3151: f64, t3153: f64, t360: f64, t39097: f64, t39103: f64, t42374: f64, t42412: f64, t42422: f64, t42428: f64, t42432: f64, t42436: f64, t4582: f64, t4588: f64, t973: f64, t974: f64) -> f64 {
    let t42444 = t976 * t10277;
    let t42459 = t42412 / 576.0_f64 - t3070 * t3071 * t3121 * t2776 / 384.0_f64 - t10403 * t3071 * t3132 * t2776 / 192.0_f64 - t3039 * t248 * t1021 * t42422 * t360 / 1024.0_f64 + 19.0_f64 / 216.0_f64 * t42428 - t42432 / 3456.0_f64 + t42436 / 288.0_f64 - t3048 * t10493 / 36.0_f64 + 5.0_f64 / 3456.0_f64 * t1041 * t4582 * t4588 * t42374 - t973 * t974 * t42444 * t39097 / 12.0_f64 - t973 * t974 * t3151 * t39103 / 48.0_f64 + t973 * t974 * t3146 * t39103 / 72.0_f64 - 11.0_f64 / 27.0_f64 * t10263 * t3153;
    t42459
}
