//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3092;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta829<F: Float>(t1210: F, t12607: F, t12628: F, t12629: F, t12658: F, t12666: F, t1277: F, t1294: F, t13166: F, t13182: F, t13183: F, t13184: F, t16750: F, t1774: F, t1775: F, t17992: F, t18005: F, t18062: F, t18065: F, t18097: F, t1828: F, t1829: F, t3576: F, t3585: F, t3732: F, t3739: F, t3791: F, t45515: F, t45522: F, t45535: F, t5220: F, t5225: F, t5246: F, t5417: F, t5423: F, t12640: F, t488: F, t17588: F, t3172: F, t3711: F, t1261: F, t17699: F, t17720: F, t3647: F, t12904: F, t5274: F, t12959: F, t17505: F) -> (F, F, F, F, F, F, F) {
        let t56687 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3092::<F>(t1210, t12607, t12628, t12629, t12658, t12666, t1277, t1294, t13166, t13182, t13183, t13184, t16750, t1774, t1775, t17992, t18005, t18062, t18065, t18097, t1828, t1829, t3576, t3585, t3732, t3739, t3791, t45515, t45522, t45535, t5220, t5225, t5246, t5417, t5423);
        let (t56707, t56713, t56718, t56720, t56727, t56728) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3093::<F>(t12640, t488, t17588, t3172, t3711, t1261, t17699, t17720, t3647, t12904, t5274, t12959, t17505);
    (t56687, t56707, t56713, t56718, t56720, t56727, t56728)
}
