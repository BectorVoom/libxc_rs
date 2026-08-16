//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta369(t10428: f64, t2414: f64, t10587: f64, t2496: f64, t10467: f64, t705: f64, t707: f64, t190: f64, t39457: f64, t706: f64, t39875: f64, t39894: f64, t9371: f64, t760: f64, t39960: f64, t39963: f64, t2523: f64, t9372: f64, t10600: f64, t14325: f64, t2258: f64, t4401: f64, t606: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40155, t40157, t40160, t40163, t40165) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344(t10428, t2414, t10587, t2496, t10467, t705, t707, t190, t39457, t706, t39875, t39894, t9371);
        let (t40167, t40169, t40171, t40173, t40175, t40178) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1345(t40165, t760, t39875, t39960, t39963, t2523, t9372, t10600, t14325, t2258, t4401, t606, t749);
    (t40155, t40157, t40160, t40163, t40165, t40167, t40169, t40171, t40173, t40175, t40178)
}
