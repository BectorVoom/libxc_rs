//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk836<F: Float>(t169: F, t242: F, t2818: F, t2929: F, t703: F, t699: F, t1102: F, t1143: F, t10810: F, t2877: F, t632: F, t2881: F, t1098: F, t1: F, t10752: F, t10780: F, t10784: F, t1132: F, t1135: F, t1139: F, t1140: F, t119: F, t159: F, t161: F, t20: F, t2782: F, t2824: F, t2869: F, t2911: F, t2917: F, t2920: F, t2923: F, t3: F, t343: F, t39: F, t477: F, t628: F, t629: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10881 = 2.4210827305188265 * t169 * t2818 * t242;
    let t10886 = t169 * t703 * t2929;
    let t10897 = 0.21223777183119583 * t169 * t699 * t2929;
    let t10906 = t169 * t1102 * t1143;
    let t10909 = t169 * t10810 * t242;
    let t10913 = 2.0752137690161367 * t169 * t2877 * t632;
    let t10918 = t169 * t2881 * t632;
    let t10922 = 0.8489510873247833 * t169 * t1098 * t1143;
    let t10953 = t10752 / 2.0 + 0.1254 * t10780 * t3 * t629 - 0.2508 * t2911 * t1135 + 0.04717548 * t10784 * t20 * t1140 + 0.39013333333333333 * t1132 * t2920 - 0.12580128 * t2917 * t2923 + 0.007532237109403992 * t477 * t39 * t161 - 0.32511111111111113 * t628 * t2869 * t161 + 0.1397792 * t1139 * t2782 * t161 - 0.015064474218807983 * t159 * t343 * t161 + 0.00011806781668990758 * t159 * t2824 * t1 * t119 * t161;
    (t10881, t10886, t10897, t10906, t10909, t10913, t10918, t10922, t10953)
}
