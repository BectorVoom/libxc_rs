//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1406/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1406(t34862: f64, t2478: f64, t6583: f64, t8272: f64, t2890: f64, t7047: f64, t10122: f64, t10564: f64, t10583: f64, t10587: f64, t1339: f64, t1391: f64, t1392: f64, t1520: f64, t1628: f64, t1641: f64, t31018: f64, t31022: f64, t31024: f64, t31557: f64, t3372: f64, t34839: f64, t34842: f64, t34855: f64, t34860: f64, t4811: f64, t541: f64, t574: f64, t587: f64, t590: f64) -> f64 {
    let t34863 = 0.59584149919750711116e-1_f64 * t34862;
    let t34865 = t6583 * t8272 * t2478;
    let t34866 = 0.38342925953920749676e0_f64 * t34865;
    let t34868 = t6583 * t2890 * t7047;
    let t34869 = 0.19171462976960374838e0_f64 * t34868;
    let t34870 = -0.11360866949309851756e0_f64 * t587 * t1391 * t1392 * t10122 - t34839 + t34842 + t31018 + t31022 + t31024 + 0.47667319935800568892e0_f64 * t10587 * t541 - 0.61348681526273199482e1_f64 * t574 * t1628 * t10583 - 0.61348681526273199482e1_f64 * t1641 * t10564 + 0.1022478025437886658e1_f64 * t4811 * t1339 * t31557 * t590 - t34855 - 0.79445533226334281487e-1_f64 * t3372 * t1520 + t34860 + t34863 - t34866 - t34869;
    t34870
}
