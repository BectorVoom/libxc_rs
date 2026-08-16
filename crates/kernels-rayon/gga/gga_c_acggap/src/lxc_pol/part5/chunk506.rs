//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 506/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk506(t2617: f64, t657: f64, t288: f64, t668: f64, t656: f64, t195: f64, t912: f64, t273: f64, t680: f64, t791: f64, t286: f64, t271: f64, t690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2618 = t2617 * t657;
    let t2620 = t668 * t288;
    let t2621 = t656 * t2620;
    let t2622 = 0.21687162600603479684e-1_f64 * t2621;
    let t2623 = t195 * t912;
    let t2624 = t656 * t2623;
    let t2625 = 0.32530743900905219526e-1_f64 * t2624;
    let t2627 = t791 * t680 * t273;
    let t2628 = t286 * t2627;
    let t2629 = 0.35089341735807877242e1_f64 * t2628;
    let t2631 = t690 * t271;
    (t2618, t2620, t2621, t2622, t2623, t2624, t2625, t2627, t2628, t2629, t2631)
}
