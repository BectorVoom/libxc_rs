//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1091;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1092;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta257<F: Float>(t3022: F, t3034: F, t3006: F, t3011: F, t4733: F, t981: F, t2935: F, t945: F, t2967: F, t941: F, t2966: F, t307: F, t302: F, t2944: F, t953: F, t2970: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11368: F, t11370: F, t11373: F, t11376: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11394, t11396, t11398, t11399, t11404, t11408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1091::<F>(t3022, t3034, t3006, t3011, t4733, t981, t2935, t945, t2967, t941, t2966, t307);
        let (t11409, t11410, t11411, t11428) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1092::<F>(t11408, t302, t2944, t953, t2970, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11443 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1093::<F>(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
    (t11394, t11396, t11398, t11399, t11404, t11408, t11409, t11410, t11411, t11428, t11443)
}
