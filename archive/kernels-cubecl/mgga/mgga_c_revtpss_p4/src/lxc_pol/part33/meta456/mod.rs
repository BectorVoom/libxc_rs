//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1658;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta456<F: Float>(t1264: F, t20272: F, t247: F, t5405: F, t6429: F, t3626: F, t6425: F, t1794: F, t5245: F, t1250: F, t3720: F, t140: F, t6652: F, t1222: F, t20795: F, t3629: F, t1261: F, t17412: F, t17444: F, t17447: F, t17453: F, t17474: F, t1808: F, t3625: F, t3647: F, t3718: F, t5331: F, t6673: F) -> (F, F, F, F, F, F, F, F) {
        let (t21153, t21157, t21161, t21164, t21166, t21169) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1658::<F>(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
        let (t21173, t21176) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1659::<F>(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
    (t21153, t21157, t21161, t21164, t21166, t21169, t21173, t21176)
}
