//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1253/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1253(t10829: f64, t1976: f64, t2874: f64, t730: f64, t10833: f64, t17474: f64, t17478: f64, t721: f64, t2849: f64, t3625: f64, t10963: f64, t723: f64) -> (f64, f64, f64, f64) {
    let t30731 = t1976 * t10829;
    let t30734 = 0.17315859105681463759e2_f64 * t730 * t30731 * t2874;
    let t30739 = 0.91082604192152556044e5_f64 * t730 * t17474 * t10833 * t17478 * t721;
    let t30742 = 0.10526802520742363173e2_f64 * t730 * t3625 * t2849;
    let t30745 = 0.14035736694323150897e2_f64 * t730 * t10963 * t723;
    (t30734, t30739, t30742, t30745)
}
