//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2710;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2711;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2712;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta700<F: Float>(t6864: F, t9918: F, t1353: F, t6816: F, t4012: F, t828: F, t3930: F, t6876: F, t1883: F, t5627: F, t13783: F, t13926: F, t6869: F, t13789: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t14053: F, t14057: F, t1410: F, t3934: F, t9977: F, t22035: F, t22065: F, t22105: F, t22140: F, t22153: F, t22176: F, t22284: F, t6862: F, t72: F, t686: F, t10023: F, t1385: F, t6888: F, t10070: F, t10074: F, t1399: F, t14191: F, t14193: F, t14203: F, t14209: F, t14255: F, t213: F, t21981: F, t22005: F, t22009: F, t22016: F, t4118: F, t546: F, t5659: F, t5675: F, t5745: F, t5755: F, t5767: F, t6874: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22285, t22287, t22289, t22292, t22294, t22295, t22298) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2710::<F>(t6864, t9918, t1353, t6816, t4012, t828, t3930, t6876, t1883, t5627, t13783, t13926, t6869);
        let (t22299, t22304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2711::<F>(t13789, t22298, t14038, t14040, t14042, t14043, t14049, t14053, t14057, t1410, t22285, t22289, t22292, t22295, t3934, t9977);
        let t22307 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2712::<F>(t22035, t22065, t22105, t22140, t22153, t22176, t22284, t22304);
        let (t22314, t22315, t22321, t22325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2713::<F>(t6862, t72, t686, t10023, t1385, t6888, t10070, t10074, t1399, t14191, t14193, t14203, t14209, t14255, t1883, t213, t21981, t22005, t22009, t22016, t22307, t4118, t546, t5659, t5675, t5745, t5755, t5767, t6874, t820);
    (t22287, t22289, t22294, t22295, t22298, t22299, t22307, t22314, t22315, t22321, t22325)
}
