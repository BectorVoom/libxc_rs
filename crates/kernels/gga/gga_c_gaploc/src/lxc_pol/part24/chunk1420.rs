//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1420/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1420<F: Float>(t35040: F, t549: F, t7025: F, t7906: F, t1339: F, t31585: F, t1537: F, t590: F, t31590: F, t10474: F, t4428: F, t10216: F, t10406: F, t10497: F, t10594: F, t10604: F, t10612: F, t1589: F, t1596: F, t1599: F, t31299: F, t3414: F, t35027: F, t35034: F, t35038: F, t4379: F, t4418: F, t4598: F, t557: F, t597: F) -> F {
    let t35041 = F::new(0.17875244975925213335e0) * t35040;
    let t35043 = t7025 * t549 * t7906;
    let t35044 = F::new(0.59584149919750711116e-1) * t35043;
    let t35045 = t1339 * t31585;
    let t35048 = F::new(0.51123901271894332902e1) * t1537 * t35045 * t590;
    let t35052 = F::new(0.51123901271894332902e1) * t1537 * t1339 * t31590 * t590;
    let t35054 = F::new(0.2044956050875773316e1) * t4428 * t10474;
    let t35067 = t35027 + F::new(0.35750489951850426669e0) * t1596 * t10497 + F::new(0.79445533226334281486e-1) * t4379 * t10612 - t35034 - t35038 + t35041 - t35044 - t35048 - t35052 + t35054 - F::new(0.47667319935800568892e0) * t557 * t1589 * t10216 - F::new(0.47667319935800568892e0) * t1599 * t10594 + F::new(0.1022478025437886658e1) * t597 * t4598 * t3414 + t31299 + F::new(0.51123901271894332905e0) * t4418 * t10406 + F::new(0.1022478025437886658e1) * t4428 * t10604;
    t35067
}
