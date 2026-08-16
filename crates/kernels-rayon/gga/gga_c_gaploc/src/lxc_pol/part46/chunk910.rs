//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 910/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk910(t2268: f64, t2854: f64, t29984: f64, t6320: f64, t41838: f64, t426: f64, t535: f64, t1063: f64, t2765: f64, t31300: f64, t2343: f64, t2787: f64, t29853: f64) -> (f64, f64, f64, f64) {
    let t42756 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t2854 * t29984;
    let t42759 = t2268 * t535 * t41838 * t426;
    let t42763 = 0.85365019907028448797e-1_f64 * t1063 * t2765 * t31300;
    let t42767 = 0.34146007962811379518e0_f64 * t2268 * t2343 * t2787 * t29853;
    (t42756, t42759, t42763, t42767)
}
