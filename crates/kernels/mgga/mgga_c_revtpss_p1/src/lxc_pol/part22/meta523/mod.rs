//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta523 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2298;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2299;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta523<F: Float>(t1169: F, t17085: F, t1179: F, t5155: F, t1719: F, t3383: F, t3386: F, t1749: F, t3520: F, t16868: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16727: F, t16748: F, t16871: F, t16876: F, t16892: F, t16708: F, t16710: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16931: F, t16933: F, t12252: F, t12261: F, t12263: F, t12265: F, t12542: F, t12543: F, t16731: F, t16852: F, t16855: F, t16858: F, t16860: F, t16863: F, t16865: F, t16887: F, t16890: F, t16895: F, t16898: F, t16901: F, t16904: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t17086, t17089, t17092, t17094, t17097, t17115, t17117, t17126) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2298::<F>(t1169, t17085, t1179, t5155, t1719, t3383, t3386, t1749, t3520, t16868, t16712, t12297, t12299, t12301, t12303, t16706, t16727, t16748, t16871, t16876);
        let (t17131, t17140, t17148) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2299::<F>(t16892, t16708, t16710, t16717, t16722, t16735, t16740, t16744, t16908, t16927, t16931, t16933);
        let t17150 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2300::<F>(t12252, t12261, t12263, t12265, t12542, t12543, t16731, t16852, t16855, t16858, t16860, t16863, t16865, t16887, t16890, t16895, t16898, t16901, t16904, t17126, t17131, t17148);
    (t17086, t17089, t17092, t17094, t17097, t17115, t17117, t17131, t17140, t17150)
}
