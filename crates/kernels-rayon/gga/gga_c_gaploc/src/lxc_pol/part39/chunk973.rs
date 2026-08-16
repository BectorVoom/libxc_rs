//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 973/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk973(t10590: f64, t2321: f64, t882: f64, t10156: f64, t1063: f64, t6750: f64, t2268: f64, t2854: f64, t29984: f64, t6320: f64, t41838: f64, t426: f64, t535: f64) -> (f64, f64, f64, f64) {
    let t42748 = t882 * t10590 * t2321;
    let t42751 = t1063 * t10156 * t6750;
    let t42756 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t2854 * t29984;
    let t42759 = t2268 * t535 * t41838 * t426;
    (t42748, t42751, t42756, t42759)
}
