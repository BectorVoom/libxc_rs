//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta494(t14220: f64, t48007: f64, t22331: f64, t2470: f64, t4101: f64, t10073: f64, t22369: f64, t136: f64, t2457: f64, t47429: f64, t6862: f64, t22351: f64, t2439: f64, t2777: f64, t22335: f64, t22361: f64, t10069: f64, t22373: f64, t10139: f64, t6874: f64, t6844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75005, t75021, t75026, t75068, t75074) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493(t14220, t48007, t22331, t2470, t4101, t10073, t22369, t136, t2457, t47429, t6862, t22351, t2439, t2777);
        let (t75092, t75113, t75119, t75123, t75128) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494(t22335, t2470, t4101, t10073, t22361, t10069, t22373, t10139, t136, t2457, t6874, t6844);
    (t75005, t75021, t75026, t75068, t75074, t75092, t75113, t75119, t75123, t75128)
}
