//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2535;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta747<F: Float>(t2435: F, t4575: F, t51973: F, t52035: F, t2852: F, t373: F, t2439: F, t4628: F, t1606: F, t9303: F, t2923: F, t4587: F, t11384: F, t1596: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t52037 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2535::<F>(t2435, t4575);
        let (t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2536::<F>(t51973, t52035, t52037, t2852, t373, t2439, t4628, t1606, t9303, t2923, t4587, t11384, t1596);
    (t52037, t52082, t52091, t52092, t52110, t52126, t52127, t52128, t52219, t52224)
}
