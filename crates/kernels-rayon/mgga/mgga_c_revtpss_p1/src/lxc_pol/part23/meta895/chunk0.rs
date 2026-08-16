//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2853/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2853(t61178: f64, t61180: f64, t39860: f64, t18263: f64, t4305: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t39818: f64, t39823: f64, t40084: f64, t49958: f64, t49964: f64, t49982: f64) -> (f64, f64, f64, f64, f64) {
    let t76976 = 12.0_f64 * t61178;
    let t76977 = 24.0_f64 * t61180;
    let t76978 = 0.56968947174242584612e-3_f64 * t39860;
    let t76979 = t18263 * t4305;
    let t76980 = 12.0_f64 * t76979;
    let t76981 = -t49958 - t49964 - t39783 - t39786 - t39791 - t39795 + t49982 + t39799 + t76976 + t39807 - t39813 + t76977 - t39818 - t39823 - t76978 + t40084 + t76980;
    (t76976, t76977, t76978, t76980, t76981)
}
