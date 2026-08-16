//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1037/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1037(t1457: f64, t8812: f64, t294: f64, t3857: f64, t914: f64, t2629: f64, t3900: f64, t2637: f64, t3894: f64, t2641: f64, t2618: f64, t3882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11221 = 0.5848223622634646207e0_f64 * t8812 * t1457;
    let t11222 = t294 * t3857;
    let t11224 = 0.11696447245269292414e1_f64 * t11222 * t914;
    let t11226 = 0.23392894490538584828e1_f64 * t2629 * t3900;
    let t11228 = 0.5848223622634646207e0_f64 * t3894 * t2637;
    let t11230 = 0.17315859105681463759e2_f64 * t3894 * t2641;
    let t11231 = t2618 * t3882;
    (t11221, t11224, t11226, t11228, t11230, t11231)
}
