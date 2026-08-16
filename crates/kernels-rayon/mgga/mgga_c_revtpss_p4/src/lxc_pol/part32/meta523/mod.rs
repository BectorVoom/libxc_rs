//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta523(t5876: f64, t670: f64, t1448: f64, t6836: f64, t6816: f64, t1868: f64, t5778: f64, t10309: f64, t607: f64, t2411: f64, t605: f64, t1955: f64, t25308: f64, t2769: f64, t7036: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64, t228: f64, t25273: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85360, t86753, t86771, t86815, t92568, t92790, t92917) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826(t5876, t670, t1448, t6836, t6816, t1868, t5778, t10309, t607, t2411, t605, t1955, t25308, t2769);
        let (t92951, t92955, t92963, t92966, t92968) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1827(t7036, t820, t844, t2482, t814, t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273);
    (t85360, t86753, t86771, t86815, t92568, t92790, t92917, t92951, t92955, t92963, t92966, t92968)
}
