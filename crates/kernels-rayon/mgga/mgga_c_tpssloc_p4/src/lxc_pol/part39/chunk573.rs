//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 573/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk573(t2617: f64, t816: f64, t809: f64, t838: f64, t842: f64, t233: f64, t813: f64, t236: f64, t240: f64, t812: f64, t828: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2618 = t2617 * t816;
    let t2621 = t809 * t838;
    let t2623 = t2617 * t842;
    let t2627 = 1.0_f64 / t813 / t233;
    let t2628 = t2627 * t236;
    let t2629 = t2628 * t240;
    let t2630 = t812 * t2629;
    let t2631 = t828 * t828;
    let t2632 = t232 * t232;
    (t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2631, t2632)
}
