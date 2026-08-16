//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta497(t23384: f64, t689: f64, t779: f64, t14987: f64, t18797: f64, t23388: f64, t786: f64, t789: f64, t23414: f64, t23413: f64, t41070: f64, t686: f64, t72: f64, t18805: f64, t50208: f64, t4321: f64, t6049: f64, t4481: f64, t63084: f64, t1580: f64, t18316: f64, t14480: f64, t252: f64, t2782: f64, t6071: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75950, t75956, t75961, t75974, t75978) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497(t23384, t689, t779, t14987, t18797, t23388, t786, t789, t23414, t23413, t41070, t686, t72);
        let (t75984, t75998, t76010, t76020, t76026) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1498(t18805, t50208, t4321, t6049, t689, t4481, t63084, t1580, t18316, t14480, t252, t2782, t6071);
    (t75950, t75956, t75961, t75974, t75978, t75984, t75998, t76010, t76020, t76026)
}
