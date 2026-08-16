//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3092;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta829(t1210: f64, t12607: f64, t12628: f64, t12629: f64, t12658: f64, t12666: f64, t1277: f64, t1294: f64, t13166: f64, t13182: f64, t13183: f64, t13184: f64, t16750: f64, t1774: f64, t1775: f64, t17992: f64, t18005: f64, t18062: f64, t18065: f64, t18097: f64, t1828: f64, t1829: f64, t3576: f64, t3585: f64, t3732: f64, t3739: f64, t3791: f64, t45515: f64, t45522: f64, t45535: f64, t5220: f64, t5225: f64, t5246: f64, t5417: f64, t5423: f64, t12640: f64, t488: f64, t17588: f64, t3172: f64, t3711: f64, t1261: f64, t17699: f64, t17720: f64, t3647: f64, t12904: f64, t5274: f64, t12959: f64, t17505: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t56687 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3092(t1210, t12607, t12628, t12629, t12658, t12666, t1277, t1294, t13166, t13182, t13183, t13184, t16750, t1774, t1775, t17992, t18005, t18062, t18065, t18097, t1828, t1829, t3576, t3585, t3732, t3739, t3791, t45515, t45522, t45535, t5220, t5225, t5246, t5417, t5423);
        let (t56707, t56713, t56718, t56720, t56727, t56728) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3093(t12640, t488, t17588, t3172, t3711, t1261, t17699, t17720, t3647, t12904, t5274, t12959, t17505);
    (t56687, t56707, t56713, t56718, t56720, t56727, t56728)
}
