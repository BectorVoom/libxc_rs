//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2718;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta842<F: Float>(t12915: F, t17344: F, t20747: F, t247: F, t1261: F, t44693: F, t6421: F, t12910: F, t12916: F, t20857: F, t1208: F, t21332: F, t225: F, t480: F, t17289: F, t1803: F, t1222: F, t6652: F, t697: F, t17709: F, t20958: F, t1235: F, t371: F, t6645: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t70129, t70133, t70140, t70208) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2718::<F>(t12915, t17344, t20747, t247, t1261, t44693, t6421, t12910, t12916, t20857, t1208, t21332);
        let (t70209, t70210, t70221, t70225, t70250, t70263) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2719::<F>(t225, t70208, t480, t17289, t1803, t1222, t6652, t697, t12916, t17709, t20958, t1235, t371, t6645, t676);
    (t70129, t70133, t70140, t70208, t70209, t70210, t70221, t70225, t70250, t70263)
}
