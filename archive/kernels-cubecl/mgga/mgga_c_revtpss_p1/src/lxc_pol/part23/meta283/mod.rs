//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta283<F: Float>(t10867: F, t251: F, t2777: F, t2789: F, t2439: F, t2435: F, t2790: F, t2778: F, t9303: F, t871: F, t9292: F, t9646: F) -> (F, F, F, F, F, F, F) {
        let (t10952, t10963, t10964, t10966, t10969, t10971, t10981) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1508::<F>(t10867, t251, t2777, t2789, t2439, t2435, t2790, t2778, t9303, t871, t9292, t9646);
    (t10952, t10963, t10964, t10966, t10969, t10971, t10981)
}
