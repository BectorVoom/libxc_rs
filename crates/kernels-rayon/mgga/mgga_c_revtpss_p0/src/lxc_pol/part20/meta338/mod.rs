//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1262;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta338(t136: f64, t243: f64, t220: f64, t10769: f64, t828: f64, t2746: f64, t240: f64, t849: f64, t10868: f64, t241: f64, t820: f64, t231: f64, t2394: f64, t2719: f64, t844: f64, t2482: f64, t814: f64, t11509: f64, t2988: f64, t4900: f64, t999: f64, t4894: f64, t245: f64, t4890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14686, t14785, t14791, t14832, t14894, t14917) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1262(t136, t243, t220, t10769, t828, t2746, t240, t849, t10868, t241, t820, t231, t2394);
        let (t14923, t14931, t15542, t15604, t15609, t15687) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263(t2719, t820, t844, t2482, t814, t11509, t2988, t4900, t999, t4894, t245, t4890);
    (t14686, t14785, t14791, t14832, t14894, t14917, t14923, t14931, t15542, t15604, t15609, t15687)
}
