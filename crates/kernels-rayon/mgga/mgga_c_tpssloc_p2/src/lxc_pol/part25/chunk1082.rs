//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1082/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1082(t22674: f64, t22892: f64, t22916: f64, t22716: f64, t6908: f64, t22751: f64, t22930: f64, t22917: f64, t22723: f64, t22891: f64, t22920: f64, t12437: f64, t1985: f64, t6889: f64, t6906: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80659 = t22892 * t22674 * t22916;
    let t80663 = t22716 * t6908;
    let t80665 = t22751 * t22930;
    let t80667 = t22751 * t22917;
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80675 = t1985 * t6889 * t6906 * t12437;
    (t80659, t80663, t80665, t80667, t80670, t80671, t80675)
}
