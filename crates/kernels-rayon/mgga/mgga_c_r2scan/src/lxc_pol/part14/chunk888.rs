//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 888/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk888(t6194: f64, t8123: f64, t6191: f64, t6212: f64, t921: f64, t6211: f64, t6209: f64, t6086: f64, t8089: f64, t6535: f64, t2133: f64, t2139: f64, t2184: f64, t2614: f64, t2636: f64, t2640: f64, t2656: f64, t6310: f64, t6324: f64, t6333: f64, t6346: f64, t6352: f64, t6386: f64, t6425: f64, t6465: f64, t6493: f64, t6583: f64, t8103: f64, t8107: f64, t8112: f64, t8119: f64) -> (f64, f64, f64) {
    let t8124 = t8123 * t6194;
    let t8125 = t6191 * t8124;
    let t8128 = t6212 * t921;
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    let t8141 = 0.23287303101564395622e-1_f64 * t6535 * t8139;
    let t8142 = 0.43341108700271342816e-1_f64 * t2133 * t8103 + 0.13002332610081402845e0_f64 * t2139 * t8107 - 0.86682217400542685632e-1_f64 * t6583 * t8112 - t6310 + t6324 + t6333 + 0.2600466522016280569e0_f64 * t6425 * t2614 + 0.17336443480108537126e0_f64 * t2184 * t8119 - 0.84755945902752848174e0_f64 * t6346 - 0.14457274399185490173e-3_f64 * t8125 + 0.64025200389650807209e-1_f64 * t6352 - 0.63479958930231934629e-2_f64 * t8130 + 0.17336443480108537126e0_f64 * t6465 * t2636 + 0.10401866088065122276e1_f64 * t6493 * t2640 + 0.11557628986739024751e0_f64 * t6386 + 0.2600466522016280569e0_f64 * t6425 * t2656 + t8141;
    (t8128, t8129, t8142)
}
