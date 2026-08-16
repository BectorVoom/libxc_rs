//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 143/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk143(t221: f64, t458: f64, t225: f64, t466: f64, t68: f64, t358: f64, t425: f64, t453: f64, t455: f64) -> (f64, f64, f64, f64) {
    let t467 = t221 * t458;
    let t470 = t466 * t225;
    let t471 = t470 * t68;
    let t475 = f64::exp(-(-t425 + t453 + t455) * t225 * t358);
    (t467, t470, t471, t475)
}
