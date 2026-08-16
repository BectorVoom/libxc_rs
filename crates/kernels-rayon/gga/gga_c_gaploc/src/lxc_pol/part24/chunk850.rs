//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 850/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk850(t4585: f64, t993: f64, t189: f64, t7861: f64, t188: f64, t1007: f64, t4598: f64, t531: f64, t7879: f64, t1004: f64, t1417: f64, t1424: f64, t1441: f64, t1537: f64, t1599: f64, t193: f64, t2828: f64, t2887: f64, t4425: f64, t4428: f64, t4631: f64, t4819: f64, t557: f64, t574: f64, t6897: f64, t6900: f64, t6909: f64, t6912: f64, t6917: f64, t8262: f64, t8266: f64, t8269: f64, t8273: f64, t8278: f64) -> (f64, f64) {
    let t8286 = t4585 * t993;
    let t8289 = t189 * t7861;
    let t8290 = t188 * t8289;
    let t8297 = t4598 * t1007;
    let t8300 = t531 * t7879;
    let t8303 = -0.51123901271894332905e0_f64 * t4425 * t2887 + 0.79445533226334281486e-1_f64 * t8262 * t1417 - 0.79445533226334281486e-1_f64 * t8266 * t1424 - 0.79445533226334281486e-1_f64 * t4819 * t8269 - 0.1022478025437886658e1_f64 * t1537 * t8273 + 0.1022478025437886658e1_f64 * t4428 * t2887 + 0.1022478025437886658e1_f64 * t1441 * t8278 - 0.38342925953920749676e0_f64 * t6897 - 0.51123901271894332902e0_f64 * t6900 - 0.76685851907841499352e0_f64 * t6909 + 0.19171462976960374838e1_f64 * t6912 - 0.11502877786176224903e1_f64 * t6917 + 0.79445533226334281487e-1_f64 * t557 * t8286 + 0.35750489951850426669e0_f64 * t8290 * t193 - 0.35750489951850426669e0_f64 * t4631 * t1004 - 0.71500979903700853338e0_f64 * t1599 * t2828 - 0.1022478025437886658e1_f64 * t574 * t8297 - 0.35750489951850426669e0_f64 * t557 * t8300;
    (t8289, t8303)
}
