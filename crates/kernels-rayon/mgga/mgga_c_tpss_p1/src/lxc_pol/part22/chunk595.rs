//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 595/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk595(t2629: f64, t914: f64, t2593: f64, t2595: f64, t904: f64, t912: f64, t2613: f64, t895: f64, t2618: f64, t2621: f64, t2464: f64, t928: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2631 = 0.11696447245269292414e1_f64 * t2629 * t914;
    let t2633 = t2593 * t2595 * t904;
    let t2635 = 0.11696447245269292414e1_f64 * t912 * t2633;
    let t2637 = t895 * t2613 * t904;
    let t2639 = 0.5848223622634646207e0_f64 * t912 * t2637;
    let t2640 = t2618 * t2595;
    let t2641 = t2640 * t2621;
    let t2643 = 0.17315859105681463759e2_f64 * t912 * t2641;
    let t2644 = t928 * t2464;
    (t2631, t2633, t2635, t2637, t2639, t2641, t2643, t2644)
}
