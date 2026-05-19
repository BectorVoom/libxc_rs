//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1285/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1285<F: Float>(t93572: F, t93606: F, t93641: F, t93678: F, t93710: F, t93747: F, t93786: F, t93824: F, t1000: F, t1089: F, t1096: F, t11620: F, t225: F, t25461: F, t25484: F, t25616: F, t25617: F, t25626: F, t25634: F, t25647: F, t25671: F, t25681: F, t25695: F, t25699: F, t3043: F, t3075: F, t3076: F, t3151: F, t3271: F, t3304: F, t3318: F, t342: F, t385: F, t7137: F, t7144: F, t7145: F, t7151: F, t7152: F, t7160: F, t7167: F, t7168: F, t7170: F, t7174: F, t93516: F, t93521: F, t93528: F) -> (F, F) {
    let t93827 = t93572 + t93606 + t93641 + t93678 + t93710 + t93747 + t93786 + t93824;
    let t93852 = -F::cast_from(0.26020884564615598386e1_f64) * t25671 * t93516 * t3151 * t3304 - F::cast_from(0.26020884564615598386e1_f64) * t93521 * t7170 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t7168 * t11620 * t1089 - F::cast_from(0.19756347548806534796e1_f64) * t93528 * t1000 + F::cast_from(0.52041769129231196772e1_f64) * t25461 * t25617 - F::cast_from(0.19756347548806534796e1_f64) * t25695 * t3076 + F::cast_from(0.10408353825846239354e2_f64) * t7144 * t7160 * t25647 * t1096 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t93827 * t225 * t385 - F::cast_from(0.10408353825846239354e2_f64) * t7151 * t7160 * t25616 * t1096 + F::cast_from(0.26020884564615598386e1_f64) * t25461 * t25484 - F::cast_from(0.26020884564615598386e1_f64) * t25626 * t7174 + F::cast_from(0.19756347548806534796e1_f64) * t3043 * t7137 + F::cast_from(0.13010442282307799193e1_f64) * t25671 * t25681 * t3151 * t3318 + F::cast_from(0.39512695097613069591e1_f64) * t25634 * t3271 - F::cast_from(0.78062653693846795158e1_f64) * t25699 * t7145 * t7152 * t3075;
    (t93827, t93852)
}
