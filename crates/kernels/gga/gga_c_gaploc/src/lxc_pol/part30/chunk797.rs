//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 797/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk797<F: Float>(t188: F, t8289: F, t1007: F, t4598: F, t531: F, t7879: F, t1004: F, t1417: F, t1424: F, t1441: F, t1537: F, t1599: F, t193: F, t2828: F, t2887: F, t4425: F, t4428: F, t4631: F, t4819: F, t557: F, t574: F, t6897: F, t6900: F, t6909: F, t6912: F, t6917: F, t8262: F, t8266: F, t8269: F, t8273: F, t8278: F, t8286: F) -> (F,) {
    let t8290 = t188 * t8289;
    let t8297 = t4598 * t1007;
    let t8300 = t531 * t7879;
    let t8303 = -0.51123901271894332905e0 * t4425 * t2887 + 0.79445533226334281486e-1 * t8262 * t1417 - 0.79445533226334281486e-1 * t8266 * t1424 - 0.79445533226334281486e-1 * t4819 * t8269 - 0.1022478025437886658e1 * t1537 * t8273 + 0.1022478025437886658e1 * t4428 * t2887 + 0.1022478025437886658e1 * t1441 * t8278 - 0.38342925953920749676e0 * t6897 - 0.51123901271894332902e0 * t6900 - 0.76685851907841499352e0 * t6909 + 0.19171462976960374838e1 * t6912 - 0.11502877786176224903e1 * t6917 + 0.79445533226334281487e-1 * t557 * t8286 + 0.35750489951850426669e0 * t8290 * t193 - 0.35750489951850426669e0 * t4631 * t1004 - 0.71500979903700853338e0 * t1599 * t2828 - 0.1022478025437886658e1 * t574 * t8297 - 0.35750489951850426669e0 * t557 * t8300;
    (t8303,)
}
