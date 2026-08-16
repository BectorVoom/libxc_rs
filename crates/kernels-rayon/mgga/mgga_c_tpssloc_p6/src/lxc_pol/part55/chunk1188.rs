//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1188/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1188(t118690: f64, t22986: f64, t6646: f64, t829: f64, t112968: f64, t25261: f64, t2647: f64, t112974: f64, t32849: f64, t814: f64, t32826: f64, t6562: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118694 = 0.3289868133696452873e-1_f64 * t22986 * t6646 * t118690 * t829;
    let t118695 = 0.76763589786250567036e-1_f64 * t112968;
    let t118699 = 0.3289868133696452873e-1_f64 * t22986 * t6646 * t25261 * t2647;
    let t118700 = 0.38381794893125283518e-1_f64 * t112974;
    let t118705 = t814 * t32849;
    let t118709 = t6562 * t794 * t32826;
    (t118694, t118695, t118699, t118700, t118705, t118709)
}
