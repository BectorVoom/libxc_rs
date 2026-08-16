//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta223(t10665: f64, t231: f64, t243: f64, t816: f64, t9707: f64, t813: f64, t2394: f64, t2476: f64, t236: f64, t807: f64, t2689: f64, t2694: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t10666, t10673, t10674, t10675, t10676, t10678) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1013(t10665, t231, t243, t816, t9707, t813, t2394, t2476, t236, t807, t2689, t2694);
    (t10666, t10673, t10674, t10675, t10676, t10678)
}
