//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 537/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk537(t1108: f64, t721: f64, t1833: f64, t1883: f64, t1962: f64, t1967: f64, t2730: f64, t2741: f64, t2755: f64, t2760: f64, t2766: f64, t2768: f64, t2772: f64, t2776: f64, t2780: f64) -> (f64, f64) {
    let t2834 = t1108 * t721;
    let t2848 = -0.1294625e1_f64 * t2755 + 0.258925e1_f64 * t2760 + t1962 - 0.301925e0_f64 * t1833 - 0.301925e0_f64 * t2730 + 0.905775e0_f64 * t2741 + 0.82524375e-1_f64 * t2766 + 0.16504875e0_f64 * t2768 + t1967 - 0.16557e0_f64 * t1883 - 0.16557e0_f64 * t2772 + 0.248355e0_f64 * t2776 + 0.248355e0_f64 * t2780;
    (t2834, t2848)
}
