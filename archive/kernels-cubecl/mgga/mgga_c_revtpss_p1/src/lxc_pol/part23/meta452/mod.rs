//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1881;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta452<F: Float>(t3011: F, t6205: F, t4733: F, t981: F, t15258: F, t4732: F, t4719: F, t4729: F, t19136: F, t19143: F, t19145: F, t19149: F, t19152: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F, t19466: F) -> (F, F, F, F, F, F, F) {
        let (t19467, t19468, t19470, t19471, t19473, t19475, t19476) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1881::<F>(t3011, t6205, t4733, t981, t15258, t4732, t4719, t4729, t19136, t19143, t19145, t19149, t19152, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337);
        let t19477 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1882::<F>(t19466, t19476);
    (t19467, t19468, t19470, t19471, t19473, t19475, t19477)
}
