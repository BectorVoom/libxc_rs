//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta915 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta915<F: Float>(t11988: F, t4834: F, t15731: F, t3124: F, t11933: F, t15794: F, t3115: F, t42793: F, t4911: F, t11951: F, t4858: F, t11922: F, t15906: F, t15909: F, t16067: F, t16069: F, t11200: F, t380: F, t16088: F, t3105: F, t4797: F, t15725: F, t15827: F, t11921: F, t16152: F, t247: F, t4837: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55272, t55279, t55290, t55293, t55320, t55325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123::<F>(t11988, t4834, t15731, t3124, t11933, t15794, t3115, t42793, t4911, t11951, t4858, t11922, t15906, t15909);
        let (t55328, t55330, t55331, t55356, t55361, t55367) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124::<F>(t11922, t16067, t16069, t11200, t380, t16088, t3105, t4797, t15725, t15827, t11921, t16152, t247, t4837);
    (t55272, t55279, t55290, t55293, t55320, t55325, t55328, t55330, t55331, t55356, t55361, t55367)
}
