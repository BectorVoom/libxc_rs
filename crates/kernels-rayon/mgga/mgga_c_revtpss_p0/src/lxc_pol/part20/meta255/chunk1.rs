//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1089/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1089(t11372: f64, t2889: f64, t2897: f64, t918: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64) -> (f64, f64, f64, f64) {
    let t11373 = t11372 * t2889;
    let t11375 = t2897 * t918;
    let t11376 = t11375 * t2889;
    let t11378 = -0.59793333333333333333e0_f64 * t11138 + 0.11958666666666666667e1_f64 * t11153 + 0.142419375e1_f64 * t11356 - 0.76790625e-1_f64 * t11359 - 0.39862222222222222223e0_f64 * t11134 + 0.29896666666666666667e0_f64 * t11140 + 0.19931111111111111111e0_f64 * t11136 - 0.33218518518518518518e0_f64 * t11147 - 0.29896666666666666667e0_f64 * t11171 - 0.27385555555555555556e0_f64 * t11366 + 0.16431333333333333333e0_f64 * t11368 + 0.1898925e1_f64 * t11370 - 0.28483875e1_f64 * t11373 + 0.46074375e0_f64 * t11376;
    (t11373, t11375, t11376, t11378)
}
