//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1991/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991(t100572: f64, t101226: f64, t101832: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t193: f64, t202: f64, t24191: f64, t24339: f64, t24344: f64, t2522: f64, t26740: f64, t26744: f64, t28248: f64, t29106: f64, t29125: f64, t4255: f64, t4303: f64, t4314: f64, t46341: f64, t5660: f64, t67123: f64, t7114: f64, t776: f64, t7845: f64, t84766: f64, t868: f64, t870: f64, t92276: f64, t97999: f64, t98003: f64, t98102: f64) -> f64 {
    let t101937 = t193 * t202 * t101832 * t870 + 6.0_f64 * t2522 * t26740 * t1484 + 6.0_f64 * t46341 * t29125 + 2.0_f64 * t1877 * t24344 * t98102 - t1877 * t24339 * t5660 + 12.0_f64 * t4314 * t7845 * t4255 - 6.0_f64 * t1877 * t84766 * t97999 - 6.0_f64 * t2522 * t26744 * t16596 - 2.0_f64 * t1877 * t92276 * t1530 - t1877 * t101226 * t868 + 6.0_f64 * t2522 * t24344 * t98003 + 3.0_f64 * t2522 * t29106 * t776 - 6.0_f64 * t2522 * t24339 * t28248 + 12.0_f64 * t24191 * t100572 - 3.0_f64 * t2522 * t7114 * t67123 - 2.0_f64 * t1877 * t26744 * t4303;
    t101937
}
