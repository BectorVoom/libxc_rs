//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1070/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1070(t4016: f64, t990: f64, t2776: f64, t1482: f64, t2804: f64, t2723: f64, t9081: f64, t9095: f64, t1464: f64, t2768: f64, t3949: f64, t975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11725 = t4016 * t990;
    let t11726 = t2776 * t11725;
    let t11730 = t2776 * t1482 * t2804;
    let t11733 = t9081 * t2723;
    let t11743 = t9095 * t2723;
    let t11750 = t2768 * t1464;
    let t11753 = t975 * t3949;
    (t11726, t11730, t11733, t11743, t11750, t11753)
}
