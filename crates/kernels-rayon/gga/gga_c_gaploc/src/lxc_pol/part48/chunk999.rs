//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 999/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk999(t13332: f64, t501: f64, t605: f64, t11718: f64, t7324: f64, t13343: f64, t17293: f64, t13581: f64, t13718: f64, t1955: f64, t45997: f64, t46000: f64, t46001: f64, t46004: f64, t46006: f64, t46008: f64, t46011: f64, t46013: f64, t46016: f64, t46019: f64, t46023: f64, t46025: f64, t46828: f64, t5552: f64, t841: f64) -> (f64, f64, f64) {
    let t46829 = t13332 * t501;
    let t46830 = t46829 * t605;
    let t46832 = 2.0_f64 * t7324 * t11718;
    let t46835 = 24.0_f64 * t17293 * t13343 * t605;
    let t46836 = 4.0_f64 * t13581 * t5552 - t13718 * t1955 - t46001 * t841 - t45997 - t46000 + t46004 - t46006 + t46008 + t46011 + t46013 - t46016 + t46019 - t46023 - t46025 + t46828 + t46830 + t46832 - t46835;
    (t46830, t46835, t46836)
}
