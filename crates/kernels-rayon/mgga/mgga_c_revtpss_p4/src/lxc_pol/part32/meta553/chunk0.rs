//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1871/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1871(t1032: f64, t5710: f64, t1426: f64, t7063: f64, t1892: f64, t25877: f64, t1955: f64, t25981: f64, t5677: f64, t820: f64, t844: f64, t241: f64, t94491: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97960 = t5710 * t1032;
    let t97961 = t97960 * t1426;
    let t97962 = t7063 * t97961;
    let t98040 = t7063 * t1892;
    let t98041 = t98040 * t25877;
    let t98050 = t1955 * t97960;
    let t98108 = t820 * t25981 * t844 * t5677;
    let t98115 = t820 * t94491 * t241;
    (t97961, t97962, t98040, t98041, t98050, t98108, t98115)
}
