//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1091/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1091(t12020: f64, t225: f64, t22723: f64, t22891: f64, t117: f64, t5247: f64, t6559: f64, t22684: f64, t6546: f64, t131: f64, t1365: f64, t1878: f64, t209: f64) -> (f64, f64, f64, f64, f64) {
    let t80640 = t225 * t12020;
    let t80670 = t22723 * t22891;
    let t80681 = t6559 * t5247 * t117;
    let t80727 = t6546 * t22684;
    let t80730 = t1365 * t131;
    let t80732 = t1878 * t80730 * t209;
    (t80640, t80670, t80681, t80727, t80732)
}
