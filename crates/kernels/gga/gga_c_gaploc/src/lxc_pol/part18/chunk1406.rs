//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1406/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1406<F: Float>(t34862: F, t2478: F, t6583: F, t8272: F, t2890: F, t7047: F, t10122: F, t10564: F, t10583: F, t10587: F, t1339: F, t1391: F, t1392: F, t1520: F, t1628: F, t1641: F, t31018: F, t31022: F, t31024: F, t31557: F, t3372: F, t34839: F, t34842: F, t34855: F, t34860: F, t4811: F, t541: F, t574: F, t587: F, t590: F) -> F {
    let t34863 = F::new(0.59584149919750711116e-1) * t34862;
    let t34865 = t6583 * t8272 * t2478;
    let t34866 = F::new(0.38342925953920749676e0) * t34865;
    let t34868 = t6583 * t2890 * t7047;
    let t34869 = F::new(0.19171462976960374838e0) * t34868;
    let t34870 = -F::new(0.11360866949309851756e0) * t587 * t1391 * t1392 * t10122 - t34839 + t34842 + t31018 + t31022 + t31024 + F::new(0.47667319935800568892e0) * t10587 * t541 - F::new(0.61348681526273199482e1) * t574 * t1628 * t10583 - F::new(0.61348681526273199482e1) * t1641 * t10564 + F::new(0.1022478025437886658e1) * t4811 * t1339 * t31557 * t590 - t34855 - F::new(0.79445533226334281487e-1) * t3372 * t1520 + t34860 + t34863 - t34866 - t34869;
    t34870
}
