//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta871 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta871<F: Float>(t1561: F, t40360: F, t14843: F, t40864: F, t10779: F, t14931: F, t1548: F, t2724: F, t10811: F, t14693: F, t2682: F, t2719: F, t4368: F, t820: F, t10778: F, t221: F, t10777: F, t14792: F, t2659: F, t4503: F, t816: F, t14803: F, t50769: F, t4372: F, t9784: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51104, t51106, t51110, t51112, t51121) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031::<F>(t1561, t40360, t14843, t40864, t10779, t14931, t1548, t2724, t10811, t14693, t2682, t2719, t4368, t820);
        let (t51123, t51125, t51133, t51135, t51168, t51170) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3032::<F>(t10778, t221, t10777, t14792, t2659, t4503, t816, t14803, t50769, t14931, t4372, t9784);
    (t51104, t51106, t51110, t51112, t51121, t51123, t51125, t51133, t51135, t51168, t51170)
}
