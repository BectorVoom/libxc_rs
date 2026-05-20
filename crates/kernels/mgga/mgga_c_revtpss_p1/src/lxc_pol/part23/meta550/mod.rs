//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2104;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2105;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2106;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta550<F: Float>(t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t22255: F, t22260: F, t22264: F, t22268: F, t22271: F, t22276: F, t22281: F, t5671: F, t9953: F, t6864: F, t9918: F, t1353: F, t6816: F, t4012: F, t828: F, t3930: F, t6876: F, t1883: F, t5627: F, t13783: F, t13926: F, t6869: F, t13789: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t14053: F, t14057: F, t3934: F, t9977: F, t22035: F, t22065: F, t22105: F, t22140: F, t22153: F, t22176: F) -> (F, F, F, F, F, F, F, F, F) {
        let t22284 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2104::<F>(t1388, t14013, t14024, t1410, t22179, t22183, t22255, t22260, t22264, t22268, t22271, t22276, t22281, t5671, t9953);
        let (t22285, t22287, t22289, t22292, t22294, t22295, t22298) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2105::<F>(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let (t22299, t22304) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2106::<F>(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let t22307 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2107::<F>(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304);
    (t22285, t22287, t22289, t22292, t22294, t22295, t22298, t22299, t22307)
}
