//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 847/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk847<F: Float>(t4585: F, t993: F, t189: F, t7861: F, t188: F, t1007: F, t4598: F, t531: F, t7879: F, t1004: F, t1417: F, t1424: F, t1441: F, t1537: F, t1599: F, t193: F, t2828: F, t2887: F, t4425: F, t4428: F, t4631: F, t4819: F, t557: F, t574: F, t6897: F, t6900: F, t6909: F, t6912: F, t6917: F, t8262: F, t8266: F, t8269: F, t8273: F, t8278: F) -> (F, F) {
    let t8286 = t4585 * t993;
    let t8289 = t189 * t7861;
    let t8290 = t188 * t8289;
    let t8297 = t4598 * t1007;
    let t8300 = t531 * t7879;
    let t8303 = -F::cast_from(0.51123901271894332905e0_f64) * t4425 * t2887 + F::cast_from(0.79445533226334281486e-1_f64) * t8262 * t1417 - F::cast_from(0.79445533226334281486e-1_f64) * t8266 * t1424 - F::cast_from(0.79445533226334281486e-1_f64) * t4819 * t8269 - F::cast_from(0.1022478025437886658e1_f64) * t1537 * t8273 + F::cast_from(0.1022478025437886658e1_f64) * t4428 * t2887 + F::cast_from(0.1022478025437886658e1_f64) * t1441 * t8278 - F::cast_from(0.38342925953920749676e0_f64) * t6897 - F::cast_from(0.51123901271894332902e0_f64) * t6900 - F::cast_from(0.76685851907841499352e0_f64) * t6909 + F::cast_from(0.19171462976960374838e1_f64) * t6912 - F::cast_from(0.11502877786176224903e1_f64) * t6917 + F::cast_from(0.79445533226334281487e-1_f64) * t557 * t8286 + F::cast_from(0.35750489951850426669e0_f64) * t8290 * t193 - F::cast_from(0.35750489951850426669e0_f64) * t4631 * t1004 - F::cast_from(0.71500979903700853338e0_f64) * t1599 * t2828 - F::cast_from(0.1022478025437886658e1_f64) * t574 * t8297 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t8300;
    (t8289, t8303)
}
