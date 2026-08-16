//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2556;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta761<F: Float>(t15749: F, t3224: F, t3140: F, t4743: F, t3149: F, t3160: F, t1041: F, t1670: F, t42994: F, t11988: F, t4834: F, t15731: F, t3124: F, t3115: F, t42793: F, t4911: F, t11200: F, t380: F, t16088: F, t3057: F, t4930: F, t1071: F, t15669: F, t12050: F, t15907: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55155, t55202, t55205, t55247, t55272, t55279) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2556::<F>(t15749, t3224, t3140, t4743, t3149, t3160, t1041, t1670, t42994, t11988, t4834, t15731, t3124);
        let (t55280, t55294, t55330, t55331, t55413, t55464, t55499) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2557::<F>(t55279, t3115, t42793, t4911, t11200, t380, t16088, t3057, t4930, t1071, t15669, t12050, t15907);
    (t55155, t55202, t55205, t55247, t55272, t55280, t55294, t55330, t55331, t55413, t55464, t55499)
}
