//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta350(t14600: f64, t676: f64, t836: f64, t14598: f64, t1558: f64, t879: f64, t2482: f64, t2801: f64, t1531: f64, t37: f64, t4392: f64, t72: f64, t757: f64, t73: f64, t830: f64, t1544: f64, t2475: f64, t4343: f64, t853: f64, t124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14602, t14603, t14605, t14606, t14608, t14613, t14616) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1656(t14600, t676, t836, t14598, t1558, t879, t2482, t2801, t1531, t37, t4392, t72);
        let (t14618, t14643, t14648, t14652, t14671) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1657(t14616, t757, t73, t830, t1544, t2475, t4343, t853, t124, t1558);
    (t14602, t14603, t14605, t14606, t14608, t14613, t14616, t14618, t14643, t14648, t14652, t14671)
}
