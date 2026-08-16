//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2034;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta608<F: Float>(t1294: F, t21471: F, t26921: F, t7648: F, t12627: F, t7635: F, t12587: F, t7669: F, t2155: F, t44126: F, t2028: F, t27980: F, t13790: F, t4102: F, t685: F, t72: F, t25875: F, t1444: F, t5740: F, t675: F, t94395: F, t14109: F, t25900: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97398, t97422, t97475, t97491, t97498, t97676) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2034::<F>(t1294, t21471, t26921, t7648, t12627, t7635, t12587, t7669, t2155, t44126, t2028, t27980);
        let (t97680, t97682, t97685, t97687, t97688) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2035::<F>(t13790, t4102, t685, t72, t25875, t97676, t1444, t5740, t675, t94395, t14109, t25900);
    (t97398, t97422, t97475, t97491, t97498, t97676, t97680, t97682, t97685, t97687, t97688)
}
