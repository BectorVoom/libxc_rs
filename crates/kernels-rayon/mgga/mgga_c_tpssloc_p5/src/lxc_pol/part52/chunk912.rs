//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 912/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk912(t1351: f64, t1824: f64, t3792: f64, t225: f64, t5319: f64, t5217: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t1484: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    let t16439 = t5319 * t225;
    let t16460 = t5217 * t225;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16596 = t1484 * t868;
    (t16306, t16311, t16439, t16460, t16521, t16524, t16596)
}
