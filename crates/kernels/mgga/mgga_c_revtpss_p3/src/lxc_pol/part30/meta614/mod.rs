//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta614<F: Float>(t25082: F, t49582: F, t8717: F, t2014: F, t25089: F, t28172: F, t27154: F, t95088: F, t26089: F, t5542: F, t2322: F, t28043: F, t4254: F, t1310: F, t28042: F, t651: F, t25851: F, t4248: F, t1518: F, t2319: F, t1937: F, t4292: F, t648: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98458, t98461, t98463, t98467, t98472) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118::<F>(t25082, t49582, t8717, t2014, t25089, t28172, t27154, t95088, t26089, t5542, t2322, t28043);
        let (t98474, t98477, t98483, t98484, t98486, t98487) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2119::<F>(t28043, t4254, t1310, t28042, t651, t25851, t4248, t1518, t2319, t1937, t4292, t648);
    (t98458, t98461, t98463, t98467, t98472, t98474, t98477, t98483, t98484, t98486, t98487)
}
