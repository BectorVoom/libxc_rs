//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta418<F: Float>(t18426: F, t18525: F, t4364: F, t221: F, t2485: F, t5978: F, t2484: F, t10552: F, t10554: F, t14317: F, t18261: F, t18262: F, t18265: F, t18267: F, t18300: F, t18301: F, t18308: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F) {
        let (t18527, t18531, t18532, t18534) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1802::<F>(t18426, t18525, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18527, t18531, t18532, t18534)
}
