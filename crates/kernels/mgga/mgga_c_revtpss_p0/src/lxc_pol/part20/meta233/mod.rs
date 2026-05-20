//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1030;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta233<F: Float>(t10868: F, t239: F, t820: F, t231: F, t2723: F, t10665: F, t827: F, t828: F, t10666: F, t2648: F, t2741: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F) -> (F, F, F, F, F, F, F, F) {
        let (t10870, t10871) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1030::<F>(t10868, t239, t820, t231, t2723);
        let (t10872, t10874, t10878, t10881, t10885, t10886) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1031::<F>(t10665, t10871, t827, t828, t10666, t2648, t2741, t2710, t826, t9732, t234, t2735);
    (t10870, t10871, t10872, t10874, t10878, t10881, t10885, t10886)
}
