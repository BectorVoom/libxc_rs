//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1091;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1092;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta257(t3022: f64, t3034: f64, t3006: f64, t3011: f64, t4733: f64, t981: f64, t2935: f64, t945: f64, t2967: f64, t941: f64, t2966: f64, t307: f64, t302: f64, t2944: f64, t953: f64, t2970: f64, t11132: f64, t11337: f64, t11158: f64, t11162: f64, t11167: f64, t11316: f64, t11319: f64, t11322: f64, t11326: f64, t11329: f64, t11332: f64, t11339: f64, t11343: f64, t11346: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11171: f64, t11356: f64, t11359: f64, t11366: f64, t11368: f64, t11370: f64, t11373: f64, t11376: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11394, t11396, t11398, t11399, t11404, t11408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1091(t3022, t3034, t3006, t3011, t4733, t981, t2935, t945, t2967, t941, t2966, t307);
        let (t11409, t11410, t11411, t11428) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1092(t11408, t302, t2944, t953, t2970, t11132, t11337, t11158, t11162, t11167, t11316, t11319, t11322, t11326, t11329, t11332, t11339, t11343, t11346);
        let t11443 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1093(t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368, t11370, t11373, t11376);
    (t11394, t11396, t11398, t11399, t11404, t11408, t11409, t11410, t11411, t11428, t11443)
}
