//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 785/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk785(t10542: f64, t10554: f64, t10559: f64, t10563: f64, t10566: f64, t10602: f64, t10604: f64, t10692: f64, t10700: f64, t12065: f64, t12117: f64, t1987: f64, t240: f64, t4764: f64, t5423: f64) -> f64 {
    let t12128 = 0.35089340384731224426e1_f64 * t5423 * t4764 + t10542 + t240 * (t12065 + t12117) - 0.35089340384731224426e1_f64 * t1987 * t10554 + 0.35089340384731224426e1_f64 * t1987 * t10604 - t10559 + t10563 - t10566 - t10602 - 0.1025389702100779493e4_f64 * t1987 * t10700 + 0.1038945353962551798e3_f64 * t1987 * t10692;
    t12128
}
