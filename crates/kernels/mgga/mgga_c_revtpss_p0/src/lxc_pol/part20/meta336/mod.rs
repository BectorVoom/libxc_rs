//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1259;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta336<F: Float>(t648: F, t670: F, t1353: F, t1448: F, t3829: F, t566: F, t1408: F, t240: F, t828: F, t9954: F, t3935: F, t1398: F, t241: F, t820: F, t9991: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F, t1392: F, t73: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13435, t13625, t13656, t13767, t13783, t13789, t13791) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1259::<F>(t648, t670, t1353, t1448, t3829, t566, t1408, t240, t828, t9954, t3935, t1398);
        let (t13804, t13845, t13847, t13902, t13999) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1260::<F>(t241, t820, t9991, t2482, t4000, t814, t136, t550, t220, t1392, t73, t844);
    (t13435, t13625, t13656, t13767, t13783, t13789, t13791, t13804, t13845, t13847, t13902, t13999)
}
