//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1156/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1156<F: Float>(t106787: F, t107207: F, t107226: F, t107435: F, t107566: F, t1089: F, t1646: F, t1651: F, t1652: F, t1668: F, t1695: F, t19482: F, t1976: F, t1978: F, t23598: F, t23959: F, t24061: F, t24089: F, t25591: F, t25651: F, t27642: F, t27669: F, t29740: F, t29747: F, t29875: F, t29883: F, t6235: F, t6244: F, t7144: F, t7145: F, t7151: F, t7160: F, t7167: F, t7812: F, t7817: F, t7818: F, t7822: F, t93890: F, t94053: F, t99675: F) -> (F,) {
    let t113867 = -0.52041769129231196772e1 * t99675 * t29740 - 0.26020884564615598386e1 * t27669 * t27642 * t24089 + 0.26020884564615598386e1 * t93890 * t107226 * t19482 * t1646 - 0.19756347548806534796e1 * t107566 * t1652 + 0.39512695097613069591e1 * t25651 * t24061 - 0.26020884564615598386e1 * t107435 * t7818 - 0.15612530738769359031e2 * t94053 * t7145 * t7817 * t6244 - 0.52041769129231196772e1 * t7151 * t7160 * t29883 * t1695 + 0.52041769129231196772e1 * t25591 * t7145 * t29875 * t1651 + 0.26020884564615598386e1 * t106787 * t7822 + 0.10408353825846239354e2 * t7144 * t7160 * t29747 * t1695 - 0.13010442282307799193e1 * t7167 * t107207 * t1668 * t1089 + 0.8673628188205199462e0 * t7151 * t7145 * t1976 * t23598 + 0.65854491829355115987e0 * t23959 * t1978 + 0.19756347548806534796e1 * t6235 * t7812;
    (t113867,)
}
