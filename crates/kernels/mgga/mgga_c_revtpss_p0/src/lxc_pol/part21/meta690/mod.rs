//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta690<F: Float>(t1130: F, t12393: F, t3376: F, t3432: F, t3488: F, t3495: F, t1175: F, t12485: F, t3444: F, t3476: F, t1156: F, t12469: F) -> (F, F, F, F, F, F) {
        let (t45041, t45046, t45061, t45064, t45075, t45080) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2511::<F>(t1130, t12393, t3376, t3432, t3488, t3495, t1175, t12485, t3444, t3476, t1156, t12469);
    (t45041, t45046, t45061, t45064, t45075, t45080)
}
