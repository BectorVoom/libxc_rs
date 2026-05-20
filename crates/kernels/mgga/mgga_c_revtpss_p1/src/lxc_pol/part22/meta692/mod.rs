//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2695;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta692<F: Float>(t22046: F, t3936: F, t3938: F, t5659: F, t5673: F, t5674: F, t1399: F, t125: F, t6836: F, t9955: F, t1413: F, t6816: F, t547: F, t807: F, t4011: F, t1353: F, t6883: F, t800: F, t13832: F, t13851: F, t13858: F, t3934: F, t3944: F, t9739: F, t9742: F, t9766: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22107, t22111, t22115, t22118, t22120, t22125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2695::<F>(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let (t22126, t22129, t22130, t22135, t22140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2696::<F>(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
    (t22107, t22111, t22115, t22118, t22120, t22125, t22126, t22129, t22130, t22135, t22140)
}
