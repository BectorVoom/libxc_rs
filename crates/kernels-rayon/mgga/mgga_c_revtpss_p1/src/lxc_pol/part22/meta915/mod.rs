//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta915 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta915(t11988: f64, t4834: f64, t15731: f64, t3124: f64, t11933: f64, t15794: f64, t3115: f64, t42793: f64, t4911: f64, t11951: f64, t4858: f64, t11922: f64, t15906: f64, t15909: f64, t16067: f64, t16069: f64, t11200: f64, t380: f64, t16088: f64, t3105: f64, t4797: f64, t15725: f64, t15827: f64, t11921: f64, t16152: f64, t247: f64, t4837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55272, t55279, t55290, t55293, t55320, t55325) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123(t11988, t4834, t15731, t3124, t11933, t15794, t3115, t42793, t4911, t11951, t4858, t11922, t15906, t15909);
        let (t55328, t55330, t55331, t55356, t55361, t55367) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3124(t11922, t16067, t16069, t11200, t380, t16088, t3105, t4797, t15725, t15827, t11921, t16152, t247, t4837);
    (t55272, t55279, t55290, t55293, t55320, t55325, t55328, t55330, t55331, t55356, t55361, t55367)
}
