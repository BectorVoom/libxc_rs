//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1031/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1031(t117553: f64, t117567: f64, t117590: f64, t117604: f64, t117622: f64, t117634: f64, t117648: f64, t117659: f64, t114456: f64, t114472: f64, t114513: f64, t114515: f64, t114517: f64, t114520: f64, t114525: f64, t114527: f64, t114529: f64, t114531: f64, t115978: f64, t115980: f64, t115983: f64, t2039: f64, t2363: f64, t32406: f64, t8508: f64, t85423: f64, t96316: f64) -> (f64, f64) {
    let t117662 = t117553 + t117567 + t117590 + t117604 + t117622 + t117634 + t117648 + t117659;
    let t117671 = t114513 + t114515 + t114517 + t114520 + t114456 + t8508 + t114525 + t114527 + t114529 + t114531 + 0.135e2_f64 * t85423 * t2039 + t115978 + t115980 + t114472 + t115983 + 27.0_f64 * t96316 * t2039 + 0.135e2_f64 * t32406 * t2363;
    (t117662, t117671)
}
