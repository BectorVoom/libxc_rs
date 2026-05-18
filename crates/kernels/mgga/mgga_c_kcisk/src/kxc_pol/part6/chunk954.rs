//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 954/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk954<F: Float>(t20: F, t28958: F, t780: F, t7632: F, t9226: F, t7261: F, t2014: F, t28312: F, t1775: F, t2013: F, t2634: F, t2644: F, t29847: F, t29854: F, t29862: F, t29867: F, t29870: F, t7581: F, t7591: F, t782: F, t788: F, t9169: F, t9178: F, t9189: F, t9218: F) -> F {
    let t29873 = t28958 * t20;
    let t29874 = t780 * t29873;
    let t29877 = t7632 * t9226;
    let t29878 = t7261 * t29877;
    let t29883 = t2014 * t28312;
    let t29884 = t1775 * t29883;
    let t29887 = -F::new(0.2698618307426597582e-1) * t782 * t29847 - F::new(0.43177892918825561313e0) * t2634 * t9178 - F::new(0.16191709844559585492e0) * t782 * t29854 + F::new(0.43177892918825561313e0) * t9189 * t2644 + F::new(0.1439263097294185377e0) * t7591 * t9218 + F::new(0.53972366148531951639e-1) * t2013 * t29862 - F::new(0.71963154864709268853e-1) * t7591 * t9169 + F::new(0.7915947035118019574e0) * t29867 * t788 - F::new(0.21588946459412780656e0) * t29870 * t788 - F::new(0.12313695387961363781e1) * t29874 * t788 + F::new(0.16191709844559585492e0) * t2013 * t29878 + F::new(0.2698618307426597582e-1) * t7581 * t9169 + F::new(0.89953943580886586067e-2) * t2013 * t29884;
    t29887
}
