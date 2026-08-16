//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1450;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta471(t10069: f64, t18742: f64, t10073: f64, t18738: f64, t10530: f64, t18718: f64, t2470: f64, t18761: f64, t874: f64, t18750: f64, t136: f64, t2457: f64, t2710: f64, t6041: f64, t10535: f64, t5978: f64, t2783: f64, t786: f64, t18689: f64, t2435: f64, t18688: f64, t2439: f64, t2777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62651, t62653, t62665, t62670, t62684, t62716) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1450(t10069, t18742, t10073, t18738, t10530, t18718, t2470, t18761, t874, t18750, t136, t2457, t2710, t6041);
        let (t62723, t62777, t62808, t62843, t62847) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1451(t10535, t136, t2457, t5978, t10069, t18750, t2783, t6041, t786, t18689, t2435, t18688, t2439, t2777);
    (t62651, t62653, t62665, t62670, t62684, t62716, t62723, t62777, t62808, t62843, t62847)
}
