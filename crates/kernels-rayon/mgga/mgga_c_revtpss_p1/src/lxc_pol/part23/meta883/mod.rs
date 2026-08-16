//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta883 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2794;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta883(t22351: f64, t2439: f64, t2777: f64, t22253: f64, t4101: f64, t686: f64, t72: f64, t22335: f64, t2470: f64, t10073: f64, t22361: f64, t10069: f64, t22373: f64, t10139: f64, t136: f64, t2457: f64, t6874: f64, t6844: f64, t14145: f64, t14171: f64, t1882: f64, t2482: f64, t22365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75074, t75089, t75092, t75113, t75119) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2794(t22351, t2439, t2777, t22253, t4101, t686, t72, t22335, t2470, t10073, t22361, t10069, t22373);
        let (t75123, t75128, t75141, t75145, t75147) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2795(t10139, t136, t2457, t6874, t6844, t14145, t14171, t1882, t2482, t10069, t22361, t22365);
    (t75074, t75089, t75092, t75113, t75119, t75123, t75128, t75141, t75145, t75147)
}
