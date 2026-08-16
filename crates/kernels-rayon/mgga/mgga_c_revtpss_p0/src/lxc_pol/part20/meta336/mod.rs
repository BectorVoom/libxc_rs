//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1259;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta336(t648: f64, t670: f64, t1353: f64, t1448: f64, t3829: f64, t566: f64, t1408: f64, t240: f64, t828: f64, t9954: f64, t3935: f64, t1398: f64, t241: f64, t820: f64, t9991: f64, t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t1392: f64, t73: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13435, t13625, t13656, t13767, t13783, t13789, t13791) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1259(t648, t670, t1353, t1448, t3829, t566, t1408, t240, t828, t9954, t3935, t1398);
        let (t13804, t13845, t13847, t13902, t13999) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260(t241, t820, t9991, t2482, t4000, t814, t136, t550, t220, t1392, t73, t844);
    (t13435, t13625, t13656, t13767, t13783, t13789, t13791, t13804, t13845, t13847, t13902, t13999)
}
