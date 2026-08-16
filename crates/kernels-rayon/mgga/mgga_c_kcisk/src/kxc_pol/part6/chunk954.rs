//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 954/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk954(t20: f64, t28958: f64, t780: f64, t7632: f64, t9226: f64, t7261: f64, t2014: f64, t28312: f64, t1775: f64, t2013: f64, t2634: f64, t2644: f64, t29847: f64, t29854: f64, t29862: f64, t29867: f64, t29870: f64, t7581: f64, t7591: f64, t782: f64, t788: f64, t9169: f64, t9178: f64, t9189: f64, t9218: f64) -> f64 {
    let t29873 = t28958 * t20;
    let t29874 = t780 * t29873;
    let t29877 = t7632 * t9226;
    let t29878 = t7261 * t29877;
    let t29883 = t2014 * t28312;
    let t29884 = t1775 * t29883;
    let t29887 = -0.2698618307426597582e-1_f64 * t782 * t29847 - 0.43177892918825561313e0_f64 * t2634 * t9178 - 0.16191709844559585492e0_f64 * t782 * t29854 + 0.43177892918825561313e0_f64 * t9189 * t2644 + 0.1439263097294185377e0_f64 * t7591 * t9218 + 0.53972366148531951639e-1_f64 * t2013 * t29862 - 0.71963154864709268853e-1_f64 * t7591 * t9169 + 0.7915947035118019574e0_f64 * t29867 * t788 - 0.21588946459412780656e0_f64 * t29870 * t788 - 0.12313695387961363781e1_f64 * t29874 * t788 + 0.16191709844559585492e0_f64 * t2013 * t29878 + 0.2698618307426597582e-1_f64 * t7581 * t9169 + 0.89953943580886586067e-2_f64 * t2013 * t29884;
    t29887
}
