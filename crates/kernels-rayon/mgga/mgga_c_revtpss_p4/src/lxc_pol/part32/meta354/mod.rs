//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1295;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta354(t10529: f64, t14587: f64, t2782: f64, t4469: f64, t72: f64, t686: f64, t874: f64, t1558: f64, t2811: f64, t2482: f64, t122: f64, t2723: f64, t676: f64, t836: f64, t879: f64, t2801: f64, t1531: f64, t37: f64, t4392: f64, t757: f64, t1544: f64, t2475: f64, t124: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t4423: f64, t136: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14590, t14596, t14598, t14600) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1294(t10529, t14587, t2782, t4469, t72, t686, t874, t1558, t2811, t2482, t122, t2723);
        let (t14603, t14608, t14613, t14616) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1295(t14600, t676, t836, t14598, t1558, t879, t2482, t2801, t1531, t37, t4392, t72);
        let (t14618, t14648, t14671, t14673, t14675, t14676, t14685) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1296(t14616, t757, t1544, t2475, t124, t1558, t10779, t2749, t10777, t125, t4423, t136, t243);
    (t14590, t14596, t14603, t14608, t14613, t14618, t14648, t14671, t14673, t14675, t14676, t14685)
}
