//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta335<F: Float>(t14143: F, t14144: F, t14141: F, t10069: F, t5737: F, t5710: F, t72: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F) -> (F, F, F, F, F, F, F) {
        let (t14145, t14146, t14149, t14155, t14158, t14159, t14161) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1635::<F>(t14143, t14144, t14141, t10069, t5737, t5710, t72, t1432, t686, t136, t1892, t2457, t3964);
    (t14145, t14146, t14149, t14155, t14158, t14159, t14161)
}
