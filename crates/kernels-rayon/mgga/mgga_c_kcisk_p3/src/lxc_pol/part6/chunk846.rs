//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 846/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk846(t28357: f64, t4744: f64, t10715: f64, t6817: f64, t8522: f64, t2063: f64, t7715: f64) -> (f64, f64, f64) {
    let t28358 = t28357 * t4744;
    let t28360 = 0.96490945932906628932e2_f64 * t10715 * t28358;
    let t28362 = t6817 * t8522;
    let t28368 = t7715 * t2063;
    (t28360, t28362, t28368)
}
