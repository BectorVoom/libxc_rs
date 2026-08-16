//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1295/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1295(t100681: f64, t106727: f64, t107226: f64, t107240: f64, t1089: f64, t12052: f64, t1646: f64, t1651: f64, t1652: f64, t1668: f64, t1696: f64, t23640: f64, t24178: f64, t25605: f64, t25629: f64, t27609: f64, t27616: f64, t29747: f64, t29752: f64, t29807: f64, t29812: f64, t29830: f64, t29843: f64, t29887: f64, t4781: f64, t4975: f64, t4982: f64, t6245: f64, t6393: f64, t7140: f64, t7144: f64, t7145: f64, t7168: f64, t7825: f64, t7837: f64, t93471: f64, t93502: f64, t94080: f64, t94085: f64) -> f64 {
    let t113774 = -0.78062653693846795158e1_f64 * t27609 * t29752 - 0.19756347548806534796e1_f64 * t106727 * t1652 - 0.13010442282307799193e1_f64 * t7825 * t29830 + 0.39512695097613069591e1_f64 * t100681 * t6245 - 0.19756347548806534796e1_f64 * t27616 * t6393 - 0.4336814094102599731e0_f64 * t93471 * t7168 * t23640 * t12052 - 0.19756347548806534796e1_f64 * t107240 * t1696 - 0.52041769129231196772e1_f64 * t25629 * t29747 * t1668 * t1089 - 0.52041769129231196772e1_f64 * t94080 * t107226 * t4982 * t1646 + 0.52041769129231196772e1_f64 * t94085 * t107226 * t4982 * t1651 - 0.13010442282307799193e1_f64 * t29812 * t7837 + 0.10408353825846239354e2_f64 * t93502 * t29843 * t4781 * t4975 - 0.65854491829355115987e0_f64 * t7140 * t24178 - 0.26020884564615598386e1_f64 * t7144 * t7145 * t29807 * t1646 + 0.52041769129231196772e1_f64 * t25605 * t29887 * t1668 * t1089;
    t113774
}
