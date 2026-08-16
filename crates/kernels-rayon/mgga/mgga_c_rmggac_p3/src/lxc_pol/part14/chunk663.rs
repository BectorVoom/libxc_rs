//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 663/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk663(t1356: f64, t8804: f64, t1635: f64, t880: f64, t1971: f64, t3351: f64, t2144: f64, t5898: f64, t2289: f64, t7720: f64, t2405: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8805 = t1356 * t8804;
    let t8807 = t880 * t1635;
    let t8808 = t1971 * t8807;
    let t8809 = t3351 * t8808;
    let t8811 = t2144 * t5898;
    let t8812 = t1971 * t8811;
    let t8813 = t3351 * t8812;
    let t8815 = t7720 * t2289;
    let t8817 = t290 * t2405;
    (t8805, t8808, t8809, t8812, t8813, t8815, t8817)
}
