//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1355;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1356;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta413(t41306: f64, t367: f64, t371: f64, t373: f64, t9291: f64, t2852: f64, t3154: f64, t11874: f64, t15688: f64, t11853: f64, t828: f64, t3181: f64, t675: f64, t283: f64, t66: f64, t11821: f64, t41270: f64, t11144: f64, t3252: f64, t11852: f64, t126: f64, t12166: f64, t15905: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42078, t42121, t42215, t42328, t42410, t42447) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1355(t41306, t367, t371, t373, t9291, t2852, t3154, t11874, t15688, t11853, t828, t3181, t675);
        let (t42472, t42508, t42518, t42534, t42621) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1356(t283, t2852, t66, t11821, t41270, t11144, t3252, t11852, t126, t12166, t15905, t994);
    (t42078, t42121, t42215, t42328, t42410, t42447, t42472, t42508, t42518, t42534, t42621)
}
