//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 849/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk849<F: Float>(t4585: F, t993: F, t189: F, t7861: F, t188: F, t1007: F, t4598: F, t531: F, t7879: F, t1004: F, t1417: F, t1424: F, t1441: F, t1537: F, t1599: F, t193: F, t2828: F, t2887: F, t4425: F, t4428: F, t4631: F, t4819: F, t557: F, t574: F, t6897: F, t6900: F, t6909: F, t6912: F, t6917: F, t8262: F, t8266: F, t8269: F, t8273: F, t8278: F) -> (F, F) {
    let t8286 = t4585 * t993;
    let t8289 = t189 * t7861;
    let t8290 = t188 * t8289;
    let t8297 = t4598 * t1007;
    let t8300 = t531 * t7879;
    let t8303 = -F::new(0.51123901271894332905e0) * t4425 * t2887 + F::new(0.79445533226334281486e-1) * t8262 * t1417 - F::new(0.79445533226334281486e-1) * t8266 * t1424 - F::new(0.79445533226334281486e-1) * t4819 * t8269 - F::new(0.1022478025437886658e1) * t1537 * t8273 + F::new(0.1022478025437886658e1) * t4428 * t2887 + F::new(0.1022478025437886658e1) * t1441 * t8278 - F::new(0.38342925953920749676e0) * t6897 - F::new(0.51123901271894332902e0) * t6900 - F::new(0.76685851907841499352e0) * t6909 + F::new(0.19171462976960374838e1) * t6912 - F::new(0.11502877786176224903e1) * t6917 + F::new(0.79445533226334281487e-1) * t557 * t8286 + F::new(0.35750489951850426669e0) * t8290 * t193 - F::new(0.35750489951850426669e0) * t4631 * t1004 - F::new(0.71500979903700853338e0) * t1599 * t2828 - F::new(0.1022478025437886658e1) * t574 * t8297 - F::new(0.35750489951850426669e0) * t557 * t8300;
    (t8289, t8303)
}
