//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1658;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta456(t1264: f64, t20272: f64, t247: f64, t5405: f64, t6429: f64, t3626: f64, t6425: f64, t1794: f64, t5245: f64, t1250: f64, t3720: f64, t140: f64, t6652: f64, t1222: f64, t20795: f64, t3629: f64, t1261: f64, t17412: f64, t17444: f64, t17447: f64, t17453: f64, t17474: f64, t1808: f64, t3625: f64, t3647: f64, t3718: f64, t5331: f64, t6673: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21153, t21157, t21161, t21164, t21166, t21169) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1658(t1264, t20272, t247, t5405, t6429, t3626, t6425, t1794, t5245, t1250, t3720, t140, t6652);
        let (t21173, t21176) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1659(t1222, t21169, t20795, t3629, t3626, t1261, t17412, t17444, t17447, t17453, t17474, t1808, t21153, t21157, t21161, t21166, t3625, t3647, t3718, t5331, t6673);
    (t21153, t21157, t21161, t21164, t21166, t21169, t21173, t21176)
}
