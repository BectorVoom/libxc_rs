//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1080/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1080(t14731: f64, t14800: f64, t14841: f64, t14882: f64, t294: f64, t2618: f64, t4939: f64, t3908: f64, t912: f64, t4918: f64, t914: f64, t11222: f64, t1457: f64) -> (f64, f64, f64, f64) {
    let t14885 = t294 * (t14731 + t14800 + t14841 + t14882);
    let t14886 = t2618 * t4939;
    let t14887 = t14886 * t3908;
    let t14889 = 0.17315859105681463759e2_f64 * t912 * t14887;
    let t14890 = t294 * t4918;
    let t14892 = 0.5848223622634646207e0_f64 * t14890 * t914;
    let t14894 = 0.11696447245269292414e1_f64 * t11222 * t1457;
    (t14885, t14889, t14892, t14894)
}
