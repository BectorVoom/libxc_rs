//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1295/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1295<F: Float>(t100681: F, t106727: F, t107226: F, t107240: F, t1089: F, t12052: F, t1646: F, t1651: F, t1652: F, t1668: F, t1696: F, t23640: F, t24178: F, t25605: F, t25629: F, t27609: F, t27616: F, t29747: F, t29752: F, t29807: F, t29812: F, t29830: F, t29843: F, t29887: F, t4781: F, t4975: F, t4982: F, t6245: F, t6393: F, t7140: F, t7144: F, t7145: F, t7168: F, t7825: F, t7837: F, t93471: F, t93502: F, t94080: F, t94085: F) -> F {
    let t113774 = -F::cast_from(0.78062653693846795158e1_f64) * t27609 * t29752 - F::cast_from(0.19756347548806534796e1_f64) * t106727 * t1652 - F::cast_from(0.13010442282307799193e1_f64) * t7825 * t29830 + F::cast_from(0.39512695097613069591e1_f64) * t100681 * t6245 - F::cast_from(0.19756347548806534796e1_f64) * t27616 * t6393 - F::cast_from(0.4336814094102599731e0_f64) * t93471 * t7168 * t23640 * t12052 - F::cast_from(0.19756347548806534796e1_f64) * t107240 * t1696 - F::cast_from(0.52041769129231196772e1_f64) * t25629 * t29747 * t1668 * t1089 - F::cast_from(0.52041769129231196772e1_f64) * t94080 * t107226 * t4982 * t1646 + F::cast_from(0.52041769129231196772e1_f64) * t94085 * t107226 * t4982 * t1651 - F::cast_from(0.13010442282307799193e1_f64) * t29812 * t7837 + F::cast_from(0.10408353825846239354e2_f64) * t93502 * t29843 * t4781 * t4975 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t24178 - F::cast_from(0.26020884564615598386e1_f64) * t7144 * t7145 * t29807 * t1646 + F::cast_from(0.52041769129231196772e1_f64) * t25605 * t29887 * t1668 * t1089;
    t113774
}
