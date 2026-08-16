//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2707;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2708;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta699<F: Float>(t1390: F, t22253: F, t828: F, t221: F, t4019: F, t6844: F, t4018: F, t14045: F, t6869: F, t3992: F, t2661: F, t6874: F, t22079: F, t5673: F, t5675: F, t1353: F, t6836: F, t9942: F, t1868: F, t5591: F, t4012: F, t1388: F, t14013: F, t14024: F, t1410: F, t22179: F, t22183: F, t5671: F, t9953: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22255, t22259, t22260, t22263, t22264, t22267) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2707::<F>(t1390, t22253, t828, t221, t4019, t6844, t4018, t14045, t6869, t3992, t2661, t6874);
        let (t22268, t22271, t22274, t22276, t22279, t22281) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2708::<F>(t22267, t4018, t22079, t5673, t5675, t1353, t6836, t828, t9942, t1868, t5591, t4012);
        let t22284 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2709::<F>(t1388, t14013, t14024, t1410, t22179, t22183, t22255, t22260, t22264, t22268, t22271, t22276, t22281, t5671, t9953);
    (t22255, t22259, t22263, t22267, t22271, t22274, t22276, t22279, t22281, t22284)
}
