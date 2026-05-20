//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk965;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk966;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta199<F: Float>(t10069: F, t4089: F, t138: F, t2438: F, t785: F, t10008: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t10049: F, t10062: F, t10066: F, t1437: F, t213: F, t3924: F, t4004: F, t4087: F, t4118: F, t546: F, t5745: F, t820: F, t9840: F, t9891: F, t9899: F, t1398: F, t1419: F, t4086: F, t543: F, t2782: F, t4056: F, t555: F, t9990: F, t1432: F, t2470: F, t4107: F, t1433: F, t9288: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10070, t10073) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk965::<F>(t10069, t4089, t138, t2438, t785);
        let t10076 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk966::<F>(t10073, t4089, t10008, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t10049, t10062, t10066, t10070, t1437, t213, t3924, t4004, t4087, t4118, t546, t5745, t820, t9840, t9891, t9899);
        let (t10079, t10080, t10082, t10084, t10085, t10090, t10098, t10102) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk967::<F>(t1398, t1419, t4086, t543, t2782, t4056, t555, t9990, t1432, t2470, t4107, t1433, t9288);
    (t10073, t10076, t10079, t10080, t10082, t10084, t10085, t10090, t10098, t10102)
}
