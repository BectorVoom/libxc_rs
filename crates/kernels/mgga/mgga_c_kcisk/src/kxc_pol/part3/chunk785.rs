//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 785/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk785<F: Float>(t13038: F, t334: F, t3688: F, t45: F, t1201: F, t1213: F, t12889: F, t12893: F, t12897: F, t12902: F, t12904: F, t12907: F, t12914: F, t13018: F, t13026: F, t1167: F, t3638: F) -> (F, F, F) {
    let t13039 = t13038 * t334;
    let t13042 = t45 * t3688;
    let t13045 = -0.1025389702100779493e4 * t1201 * t12889 - 0.51947267698127589897e2 * t1201 * t12893 + 0.35089340384731224426e1 * t1201 * t12897 + t12902 + t12904 + t12907 - t12914 + t13018 + t13026 + 0.19751789702565206229e-1 * t45 * t13039 - 0.17544670192365612213e1 * t13042 * t1213;
    let t13048 = t1167 * t3638;
    (t13039, t13045, t13048)
}
