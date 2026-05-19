//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 888/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk888<F: Float>(t6194: F, t8123: F, t6191: F, t6212: F, t921: F, t6211: F, t6209: F, t6086: F, t8089: F, t6535: F, t2133: F, t2139: F, t2184: F, t2614: F, t2636: F, t2640: F, t2656: F, t6310: F, t6324: F, t6333: F, t6346: F, t6352: F, t6386: F, t6425: F, t6465: F, t6493: F, t6583: F, t8103: F, t8107: F, t8112: F, t8119: F) -> (F, F, F) {
    let t8124 = t8123 * t6194;
    let t8125 = t6191 * t8124;
    let t8128 = t6212 * t921;
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    let t8141 = F::cast_from(0.23287303101564395622e-1_f64) * t6535 * t8139;
    let t8142 = F::cast_from(0.43341108700271342816e-1_f64) * t2133 * t8103 + F::cast_from(0.13002332610081402845e0_f64) * t2139 * t8107 - F::cast_from(0.86682217400542685632e-1_f64) * t6583 * t8112 - t6310 + t6324 + t6333 + F::cast_from(0.2600466522016280569e0_f64) * t6425 * t2614 + F::cast_from(0.17336443480108537126e0_f64) * t2184 * t8119 - F::cast_from(0.84755945902752848174e0_f64) * t6346 - F::cast_from(0.14457274399185490173e-3_f64) * t8125 + F::cast_from(0.64025200389650807209e-1_f64) * t6352 - F::cast_from(0.63479958930231934629e-2_f64) * t8130 + F::cast_from(0.17336443480108537126e0_f64) * t6465 * t2636 + F::cast_from(0.10401866088065122276e1_f64) * t6493 * t2640 + F::cast_from(0.11557628986739024751e0_f64) * t6386 + F::cast_from(0.2600466522016280569e0_f64) * t6425 * t2656 + t8141;
    (t8128, t8129, t8142)
}
