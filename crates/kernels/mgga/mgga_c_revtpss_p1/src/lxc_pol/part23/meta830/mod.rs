//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta830<F: Float>(t20020: F, t3224: F, t1025: F, t127: F, t19768: F, t371: F, t225: F, t64686: F, t366: F, t64907: F, t19773: F, t3215: F, t11922: F, t16067: F, t19721: F, t19566: F, t3090: F, t1086: F, t19462: F, t19972: F, t4892: F, t19658: F, t3124: F, t19882: F, t3106: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t67493, t67499, t67501, t67516, t67521) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689::<F>(t20020, t3224, t1025, t127, t19768, t371, t225, t64686, t366, t64907, t19773, t3215);
        let (t67526, t67528, t67551, t67560, t67568, t67571) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690::<F>(t11922, t16067, t19721, t19566, t3090, t1086, t19462, t19972, t4892, t19658, t3124, t19882, t3106);
    (t67493, t67499, t67501, t67516, t67521, t67526, t67528, t67551, t67560, t67568, t67571)
}
