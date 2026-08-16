//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2851/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851(t61130: f64, t10439: f64, t22688: f64, t750: f64, t49926: f64, t18263: f64, t4308: f64, t49940: f64, t23211: f64, t72: f64, t757: f64, t61165: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76963 = 12.0_f64 * t61130;
    let t76965 = t10439 * t750 * t22688;
    let t76966 = 24.0_f64 * t76965;
    let t76967 = 0.65061487801810439052e-1_f64 * t49926;
    let t76969 = 12.0_f64 * t18263 * t4308;
    let t76970 = 0.10526802520742363173e2_f64 * t49940;
    let t76972 = t23211 * t72 * t757;
    let t76973 = 0.18311447306006545054e-3_f64 * t76972;
    let t76974 = 36.0_f64 * t61165;
    (t76963, t76966, t76967, t76969, t76970, t76973, t76974)
}
