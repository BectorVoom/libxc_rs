//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta610<F: Float>(t25082: F, t49582: F, t8717: F, t2014: F, t25089: F, t28172: F, t27154: F, t95088: F, t26089: F, t5542: F, t13425: F, t13537: F, t1843: F, t2007: F, t25096: F, t28025: F, t4246: F, t4293: F, t6985: F, t7221: F, t98426: F, t98428: F, t98430: F, t98432: F, t98439: F, t98440: F, t98442: F, t98449: F, t98452: F, t98455: F, t2322: F, t28043: F, t4254: F, t1310: F, t28042: F, t651: F, t25851: F, t4248: F, t1518: F, t2319: F, t1937: F, t4292: F, t648: F) -> (F, F, F, F, F, F, F, F) {
        let t98468 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131::<F>(t25082, t49582, t8717, t2014, t25089, t28172, t27154, t95088, t26089, t5542, t13425, t13537, t1843, t2007, t25096, t28025, t4246, t4293, t6985, t7221, t98426, t98428, t98430, t98432, t98439, t98440, t98442, t98449, t98452, t98455);
        let (t98472, t98474, t98477, t98483, t98484, t98486, t98487) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2132::<F>(t2322, t28043, t4254, t1310, t28042, t651, t25851, t4248, t1518, t2319, t1937, t4292, t648);
    (t98468, t98472, t98474, t98477, t98483, t98484, t98486, t98487)
}
