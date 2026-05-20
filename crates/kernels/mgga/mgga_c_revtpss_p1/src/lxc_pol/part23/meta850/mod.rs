//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta850 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2733;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta850<F: Float>(t1250: F, t5245: F, t1794: F, t372: F, t5277: F, t17395: F, t17400: F, t20809: F, t12772: F, t21172: F, t5331: F, t3655: F, t6598: F, t6602: F, t20816: F, t3708: F, t17384: F, t17448: F, t17183: F, t17350: F, t5436: F, t17435: F, t5323: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71055, t71061, t71081, t71112, t71117, t71187) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2733::<F>(t1250, t5245, t1794, t372, t5277, t17395, t17400, t20809, t12772, t21172, t5331, t3655, t6598);
        let (t71192, t71207, t71232, t71238, t71275, t71278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2734::<F>(t3655, t6602, t20816, t3708, t17384, t17448, t17183, t17350, t17395, t5436, t17435, t5323);
    (t71055, t71061, t71081, t71112, t71117, t71187, t71192, t71207, t71232, t71238, t71275, t71278)
}
