//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 742/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk742<F: Float>(t2464: F, t6977: F, t2476: F, t2482: F, t589: F, t587: F, t1430: F, t1441: F, t1537: F, t1599: F, t2446: F, t2449: F, t2457: F, t2493: F, t4418: F, t4428: F, t4849: F, t557: F, t6937: F, t6944: F, t6950: F, t6954: F, t6957: F, t6959: F, t6961: F, t6963: F, t6965: F, t6968: F, t6972: F, t6975: F) -> (F, F, F) {
    let t6978 = t2464 * t6977;
    let t6979 = t2476 * t6978;
    let t6981 = t589 * t2482;
    let t6982 = t587 * t6981;
    let t6984 = -F::cast_from(0.47667319935800568892e0_f64) * t1599 * t2449 - F::cast_from(0.71500979903700853338e0_f64) * t1599 * t2446 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t6937 + F::cast_from(0.51123901271894332905e0_f64) * t4418 * t2493 + F::cast_from(0.1022478025437886658e1_f64) * t4428 * t2457 + F::cast_from(0.1022478025437886658e1_f64) * t1441 * t6944 - F::cast_from(0.1022478025437886658e1_f64) * t4849 * t2493 - F::cast_from(0.1022478025437886658e1_f64) * t1537 * t6950 + F::cast_from(0.23833659967900284446e0_f64) * t6954 * t1430 - F::cast_from(0.59584149919750711116e-1_f64) * t6957 + F::cast_from(0.29792074959875355558e-1_f64) * t6959 + F::cast_from(0.14896037479937677779e-1_f64) * t6961 - F::cast_from(0.14300195980740170668e1_f64) * t6963 * t6965 - F::cast_from(0.29792074959875355558e-1_f64) * t6968 - F::cast_from(0.14896037479937677779e-1_f64) * t6972 + F::cast_from(0.19171462976960374838e0_f64) * t6975 - F::cast_from(0.42603251059911944086e-1_f64) * t6979 + F::cast_from(0.51123901271894332903e0_f64) * t6982;
    (t6979, t6982, t6984)
}
