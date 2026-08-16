//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 749/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk749(t5147: f64, t970: f64, t10450: f64, t1856: f64, t10464: f64, t706: f64, t5152: f64, t960: f64, t11545: f64, t11548: f64, t11550: f64, t11553: f64, t11556: f64, t11559: f64, t11562: f64, t11564: f64, t158: f64, t165: f64, t173: f64) -> f64 {
    let t11566 = t970 * t5147;
    let t11568 = t1856 * t10450;
    let t11571 = t706 * t10464;
    let t11574 = t960 * t5152;
    let t11576 = 0.4755e-2_f64 * t165 * t11545 + 0.70578375e-4_f64 * t11548 + 0.30247875e-4_f64 * t173 * t11550 - 0.2016525e-4_f64 * t173 * t11553 + 0.3513e-2_f64 * t158 * t11556 + 0.21078e-1_f64 * t158 * t11559 + 0.117630625e-3_f64 * t11562 - 0.352891875e-4_f64 * t11564 + 0.4705225e-4_f64 * t11566 + 0.50413125e-5_f64 * t173 * t11568 + 0.22405833333333333333e-5_f64 * t173 * t11571 + 0.14052e-1_f64 * t11574;
    t11576
}
