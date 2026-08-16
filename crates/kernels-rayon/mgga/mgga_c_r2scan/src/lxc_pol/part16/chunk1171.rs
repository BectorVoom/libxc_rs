//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1171/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1171(t3105: f64, t37764: f64, t42978: f64, t42980: f64, t42982: f64, t42985: f64, t42988: f64, t42991: f64, t42994: f64, t42996: f64, t42999: f64, t43002: f64) -> f64 {
    let t43004 = t37764 * t3105;
    let t43006 = -0.23115257973478049502e0_f64 * t42978 + 0.16463622957338778996e0_f64 * t42980 + 0.10975748638225852664e0_f64 * t42982 - 0.86682217400542685632e-1_f64 * t42985 - 0.2600466522016280569e0_f64 * t42988 + 0.17336443480108537126e0_f64 * t42991 - 0.5200933044032561138e0_f64 * t42994 - 0.86682217400542685632e-1_f64 * t42996 + 0.43341108700271342816e-1_f64 * t42999 + 0.13002332610081402845e0_f64 * t43002 - 0.25610080155860322883e0_f64 * t43004;
    t43006
}
