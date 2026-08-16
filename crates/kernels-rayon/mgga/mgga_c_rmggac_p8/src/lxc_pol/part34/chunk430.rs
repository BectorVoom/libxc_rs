//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 430/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk430(t2367: f64, t874: f64, t352: f64, t570: f64, t7567: f64, t1635: f64, t880: f64, t2144: f64, t5898: f64, t2405: f64, t290: f64, t1652: f64, t2060: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8800 = t874 * t2367;
    let t8801 = t8800 * t352;
    let t8804 = t7567 * t570;
    let t8807 = t880 * t1635;
    let t8811 = t2144 * t5898;
    let t8817 = t290 * t2405;
    let t8821 = t2060 * t1652;
    (t8801, t8804, t8807, t8811, t8817, t8821)
}
