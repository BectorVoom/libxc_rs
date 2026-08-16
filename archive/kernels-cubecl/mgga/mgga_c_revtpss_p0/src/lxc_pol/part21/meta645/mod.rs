//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta645<F: Float>(t2979: F, t3011: F, t11506: F, t960: F, t315: F, t41224: F, t2935: F, t2942: F, t11408: F, t941: F, t2986: F, t11465: F) -> (F, F, F, F, F, F, F) {
        let (t41751, t41756, t41759, t41775, t41779, t41785, t41788) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2430::<F>(t2979, t3011, t11506, t960, t315, t41224, t2935, t2942, t11408, t941, t2986, t11465);
    (t41751, t41756, t41759, t41775, t41779, t41785, t41788)
}
