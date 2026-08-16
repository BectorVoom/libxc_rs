//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2070/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070(t90912: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t1338: f64, t26328: f64, t26462: f64, t6914: f64, t22705: f64, t26414: f64, t81228: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90913 = 0.76763589786250567036e-1_f64 * t90912;
    let t90914 = t22839 * t215;
    let t90915 = t80854 * t562;
    let t90952 = t1338 * t26328;
    let t90956 = t6914 * t26462;
    let t90957 = 0.38381794893125283518e-1_f64 * t90956;
    let t90961 = t81228 * t22705 * t26414;
    (t90913, t90914, t90915, t90952, t90957, t90961)
}
