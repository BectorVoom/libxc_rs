//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1285/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1285(t93572: f64, t93606: f64, t93641: f64, t93678: f64, t93710: f64, t93747: f64, t93786: f64, t93824: f64, t1000: f64, t1089: f64, t1096: f64, t11620: f64, t225: f64, t25461: f64, t25484: f64, t25616: f64, t25617: f64, t25626: f64, t25634: f64, t25647: f64, t25671: f64, t25681: f64, t25695: f64, t25699: f64, t3043: f64, t3075: f64, t3076: f64, t3151: f64, t3271: f64, t3304: f64, t3318: f64, t342: f64, t385: f64, t7137: f64, t7144: f64, t7145: f64, t7151: f64, t7152: f64, t7160: f64, t7167: f64, t7168: f64, t7170: f64, t7174: f64, t93516: f64, t93521: f64, t93528: f64) -> (f64, f64) {
    let t93827 = t93572 + t93606 + t93641 + t93678 + t93710 + t93747 + t93786 + t93824;
    let t93852 = -0.26020884564615598386e1_f64 * t25671 * t93516 * t3151 * t3304 - 0.26020884564615598386e1_f64 * t93521 * t7170 - 0.4336814094102599731e0_f64 * t7167 * t7168 * t11620 * t1089 - 0.19756347548806534796e1_f64 * t93528 * t1000 + 0.52041769129231196772e1_f64 * t25461 * t25617 - 0.19756347548806534796e1_f64 * t25695 * t3076 + 0.10408353825846239354e2_f64 * t7144 * t7160 * t25647 * t1096 + 0.65854491829355115987e0_f64 * t342 * t93827 * t225 * t385 - 0.10408353825846239354e2_f64 * t7151 * t7160 * t25616 * t1096 + 0.26020884564615598386e1_f64 * t25461 * t25484 - 0.26020884564615598386e1_f64 * t25626 * t7174 + 0.19756347548806534796e1_f64 * t3043 * t7137 + 0.13010442282307799193e1_f64 * t25671 * t25681 * t3151 * t3318 + 0.39512695097613069591e1_f64 * t25634 * t3271 - 0.78062653693846795158e1_f64 * t25699 * t7145 * t7152 * t3075;
    (t93827, t93852)
}
