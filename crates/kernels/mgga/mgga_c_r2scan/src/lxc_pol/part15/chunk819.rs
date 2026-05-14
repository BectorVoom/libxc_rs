//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 819/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk819<F: Float>(t6211: F, t8128: F, t6209: F, t6086: F, t8089: F, t6535: F, t2133: F, t2139: F, t2184: F, t2614: F, t2636: F, t2640: F, t2656: F, t6310: F, t6324: F, t6333: F, t6346: F, t6352: F, t6386: F, t6425: F, t6465: F, t6493: F, t6583: F, t8103: F, t8107: F, t8112: F, t8119: F, t8125: F) -> (F, F) {
    let t8129 = t6211 * t8128;
    let t8130 = t6209 * t8129;
    let t8139 = t6086 * t8089;
    let t8141 = 0.23287303101564395622e-1 * t6535 * t8139;
    let t8142 = 0.43341108700271342816e-1 * t2133 * t8103 + 0.13002332610081402845e0 * t2139 * t8107 - 0.86682217400542685632e-1 * t6583 * t8112 - t6310 + t6324 + t6333 + 0.2600466522016280569e0 * t6425 * t2614 + 0.17336443480108537126e0 * t2184 * t8119 - 0.84755945902752848174e0 * t6346 - 0.14457274399185490173e-3 * t8125 + 0.64025200389650807209e-1 * t6352 - 0.63479958930231934629e-2 * t8130 + 0.17336443480108537126e0 * t6465 * t2636 + 0.10401866088065122276e1 * t6493 * t2640 + 0.11557628986739024751e0 * t6386 + 0.2600466522016280569e0 * t6425 * t2656 + t8141;
    (t8129, t8142)
}
