//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1038/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1038<F: Float>(t3308: F, t6218: F, t8803: F, t11797: F, t2651: F, t10776: F, t8795: F, t10772: F, t8799: F, t3105: F, t37764: F, t42978: F, t42980: F, t42982: F, t42985: F, t42988: F, t42991: F) -> (F,) {
    let t42994 = t6218 * t3308 * t8803;
    let t42996 = t2651 * t11797;
    let t42999 = t10776 * t3308 * t8795;
    let t43002 = t10772 * t3308 * t8799;
    let t43004 = t37764 * t3105;
    let t43006 = -0.23115257973478049502e0 * t42978 + 0.16463622957338778996e0 * t42980 + 0.10975748638225852664e0 * t42982 - 0.86682217400542685632e-1 * t42985 - 0.2600466522016280569e0 * t42988 + 0.17336443480108537126e0 * t42991 - 0.5200933044032561138e0 * t42994 - 0.86682217400542685632e-1 * t42996 + 0.43341108700271342816e-1 * t42999 + 0.13002332610081402845e0 * t43002 - 0.25610080155860322883e0 * t43004;
    (t43006,)
}
