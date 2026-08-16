//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 721/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk721(t12692: f64, t12813: f64, t1010: f64, t1390: f64, t313: f64, t1336: f64, t140: f64, t3531: f64, t441: f64) -> (f64, f64, f64, f64) {
    let t12814 = t12692 + t12813;
    let t12815 = t1010 * t12814;
    let t12825 = 1.0_f64 / t313 / t1390;
    let t12827 = t140 * t1336 * t12825;
    let t12829 = 1.0_f64 / t3531 / t441;
    (t12815, t12825, t12827, t12829)
}
