//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2299;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2300;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta620<F: Float>(t225: F, t24698: F, t480: F, t1774: F, t6622: F, t1250: F, t3720: F, t6587: F, t247: F, t3719: F, t12900: F, t17629: F, t21170: F, t21189: F, t21193: F, t21216: F, t21234: F, t21249: F, t24681: F, t24684: F, t3718: F, t484: F, t5381: F, t5384: F, t6683: F) -> (F, F, F, F, F, F, F, F) {
        let (t24699, t24700, t24704) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2299::<F>(t225, t24698, t480, t1774, t6622);
        let (t24705, t24706, t24713) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2300::<F>(t1250, t24704, t3720, t1774, t6587);
        let (t24715, t24722) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2301::<F>(t247, t24713, t3719, t12900, t17629, t21170, t21189, t21193, t21216, t21234, t21249, t24681, t24684, t24700, t24706, t3718, t484, t5381, t5384, t6683);
    (t24699, t24700, t24704, t24705, t24706, t24713, t24715, t24722)
}
