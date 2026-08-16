//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta771 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2856;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta771(t13100: f64, t828: f64, t12699: f64, t3624: f64, t12879: f64, t3625: f64, t3630: f64, t1260: f64, t12975: f64, t1247: f64, t1251: f64, t42994: f64, t12904: f64, t3708: f64, t11262: f64, t3590: f64, t3610: f64, t3612: f64, t1231: f64, t12898: f64, t3651: f64, t3655: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44225, t44230, t44250, t44252, t44260, t44264) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2856(t13100, t828, t12699, t3624, t12879, t3625, t3630, t1260, t12975, t1247, t1251, t42994);
        let (t44270, t44273, t44276, t44291, t44293, t44307) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2857(t12904, t3708, t11262, t1247, t3590, t3610, t3612, t1231, t12898, t3651, t3655, t43813);
    (t44225, t44230, t44250, t44252, t44260, t44264, t44270, t44273, t44276, t44291, t44293, t44307)
}
