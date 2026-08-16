//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta610(t25082: f64, t49582: f64, t8717: f64, t2014: f64, t25089: f64, t28172: f64, t27154: f64, t95088: f64, t26089: f64, t5542: f64, t13425: f64, t13537: f64, t1843: f64, t2007: f64, t25096: f64, t28025: f64, t4246: f64, t4293: f64, t6985: f64, t7221: f64, t98426: f64, t98428: f64, t98430: f64, t98432: f64, t98439: f64, t98440: f64, t98442: f64, t98449: f64, t98452: f64, t98455: f64, t2322: f64, t28043: f64, t4254: f64, t1310: f64, t28042: f64, t651: f64, t25851: f64, t4248: f64, t1518: f64, t2319: f64, t1937: f64, t4292: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t98468 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131(t25082, t49582, t8717, t2014, t25089, t28172, t27154, t95088, t26089, t5542, t13425, t13537, t1843, t2007, t25096, t28025, t4246, t4293, t6985, t7221, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452, t98455);
        let (t98472, t98474, t98477, t98483, t98484, t98486, t98487) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2132(t2322, t28043, t4254, t1310, t28042, t651, t25851, t4248, t1518, t2319, t1937, t4292, t648);
    (t98468, t98472, t98474, t98477, t98483, t98484, t98486, t98487)
}
