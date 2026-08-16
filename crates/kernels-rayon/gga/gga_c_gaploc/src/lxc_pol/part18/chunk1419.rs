//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1419/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1419(t35040: f64, t549: f64, t7025: f64, t7906: f64, t1339: f64, t31585: f64, t1537: f64, t590: f64, t31590: f64, t10474: f64, t4428: f64, t10216: f64, t10406: f64, t10497: f64, t10594: f64, t10604: f64, t10612: f64, t1589: f64, t1596: f64, t1599: f64, t31299: f64, t3414: f64, t35027: f64, t35034: f64, t35038: f64, t4379: f64, t4418: f64, t4598: f64, t557: f64, t597: f64) -> f64 {
    let t35041 = 0.17875244975925213335e0_f64 * t35040;
    let t35043 = t7025 * t549 * t7906;
    let t35044 = 0.59584149919750711116e-1_f64 * t35043;
    let t35045 = t1339 * t31585;
    let t35048 = 0.51123901271894332902e1_f64 * t1537 * t35045 * t590;
    let t35052 = 0.51123901271894332902e1_f64 * t1537 * t1339 * t31590 * t590;
    let t35054 = 0.2044956050875773316e1_f64 * t4428 * t10474;
    let t35067 = t35027 + 0.35750489951850426669e0_f64 * t1596 * t10497 + 0.79445533226334281486e-1_f64 * t4379 * t10612 - t35034 - t35038 + t35041 - t35044 - t35048 - t35052 + t35054 - 0.47667319935800568892e0_f64 * t557 * t1589 * t10216 - 0.47667319935800568892e0_f64 * t1599 * t10594 + 0.1022478025437886658e1_f64 * t597 * t4598 * t3414 + t31299 + 0.51123901271894332905e0_f64 * t4418 * t10406 + 0.1022478025437886658e1_f64 * t4428 * t10604;
    t35067
}
