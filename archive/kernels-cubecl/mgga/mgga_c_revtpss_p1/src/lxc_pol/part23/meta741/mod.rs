//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta741 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2520;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta741<F: Float>(t14986: F, t2453: F, t10506: F, t2458: F, t4470: F, t10069: F, t14482: F, t15003: F, t41020: F, t786: F, t867: F, t14567: F, t10538: F, t213: F, t225: F, t40321: F, t14574: F, t2439: F, t2777: F, t14504: F, t14557: F, t9303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51258, t51260, t51263, t51264, t51269, t51276, t51297) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2520::<F>(t14986, t2453, t10506, t2458, t4470, t10069, t14482, t15003, t41020, t786, t867, t14567);
        let (t51299, t51320, t51355, t51374, t51390) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521::<F>(t10538, t51297, t213, t225, t40321, t14574, t2439, t2777, t10069, t14504, t14557, t9303);
    (t51258, t51260, t51263, t51264, t51269, t51276, t51297, t51299, t51320, t51355, t51374, t51390)
}
