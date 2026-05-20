//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta260<F: Float>(t3006: F, t974: F, t3014: F, t972: F, t2873: F, t910: F, t2876: F, t11300: F, t935: F, t2924: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> (F, F, F, F, F, F, F) {
        let (t11521, t11525, t11528, t11530, t11531, t11533, t11545) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1103::<F>(t3006, t974, t3014, t972, t2873, t910, t2876, t11300, t935, t2924, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
    (t11521, t11525, t11528, t11530, t11531, t11533, t11545)
}
