//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1347/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1347<F: Float>(t2893: F, t4529: F, t2895: F, t10080: F, t9792: F, t9796: F, t4077: F, t1114: F, t4545: F, t1106: F, t1111: F, t10193: F, t5395: F, t10034: F, t10036: F, t10079: F, t10156: F, t10166: F, t11822: F, t11897: F, t28056: F, t28061: F, t28351: F, t28434: F, t32614: F, t32618: F, t32624: F, t3741: F, t516: F, t535: F, t5427: F, t7818: F, t9853: F, t9887: F) -> (F, F, F, F, F, F, F) {
    let t32930 = t2893 * t4529;
    let t32931 = t32930 * t2895;
    let t32934 = t10080 * t9792;
    let t32937 = t10080 * t9796;
    let t32940 = t4077 * t2893;
    let t32941 = t32940 * t2895;
    let t32949 = t4545 * t1114;
    let t32950 = t1106 * t32949;
    let t32953 = t4545 * t1111;
    let t32954 = t1106 * t32953;
    let t32971 = t5395 * t10193;
    let t32974 = 0.16128e-4 * t7818 * t32931 + 0.576e0 * t10079 * t32934 - 0.672e0 * t11897 * t32937 + 0.11264e-4 * t3741 * t32941 + 800.0 / 9.0 * t5427 * t10034 * t10036 + 0.96e-4 * t9887 * t11822 - 0.32e1 * t10156 * t32950 + 0.32e1 * t10156 * t32954 - 0.35555555555555555556e0 * t10166 * t32950 + 0.35555555555555555556e0 * t10166 * t32954 + 0.110592e-6 * t28351 * t32624 - 0.36864e-7 * t28434 * t32614 + 0.56888888888888888888e-2 * t516 * t28061 * t32618 + 0.17066666666666666667e-1 * t535 * t28056 * t32618 - 0.10666666666666666667e-2 * t32971 * t9853;
    (t32931, t32934, t32937, t32941, t32950, t32954, t32974)
}
