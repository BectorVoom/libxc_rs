//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2556;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta761(t15749: f64, t3224: f64, t3140: f64, t4743: f64, t3149: f64, t3160: f64, t1041: f64, t1670: f64, t42994: f64, t11988: f64, t4834: f64, t15731: f64, t3124: f64, t3115: f64, t42793: f64, t4911: f64, t11200: f64, t380: f64, t16088: f64, t3057: f64, t4930: f64, t1071: f64, t15669: f64, t12050: f64, t15907: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55155, t55202, t55205, t55247, t55272, t55279) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2556(t15749, t3224, t3140, t4743, t3149, t3160, t1041, t1670, t42994, t11988, t4834, t15731, t3124);
        let (t55280, t55294, t55330, t55331, t55413, t55464, t55499) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557(t55279, t3115, t42793, t4911, t11200, t380, t16088, t3057, t4930, t1071, t15669, t12050, t15907);
    (t55155, t55202, t55205, t55247, t55272, t55280, t55294, t55330, t55331, t55413, t55464, t55499)
}
