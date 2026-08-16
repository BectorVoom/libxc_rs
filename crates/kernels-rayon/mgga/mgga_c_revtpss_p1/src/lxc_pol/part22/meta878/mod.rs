//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta878 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3046;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta878(t14575: f64, t2435: f64, t10943: f64, t14598: f64, t686: f64, t72: f64, t10541: f64, t14495: f64, t2782: f64, t10518: f64, t14568: f64, t1568: f64, t4503: f64, t786: f64, t10532: f64, t40270: f64, t4496: f64, t136: f64, t137: f64, t14597: f64, t2438: f64, t2723: f64, t49180: f64, t836: f64, t2457: f64, t2710: f64, t4469: f64, t2722: f64, t50474: f64, t39597: f64, t14586: f64, t10529: f64, t10115: f64, t1576: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51537, t51541, t51544, t51546, t51548) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045(t14575, t2435, t10943, t14598, t686, t72, t10541, t14495, t2782, t10518, t14568, t1568, t4503);
        let (t51550, t51553, t51560) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3046(t51548, t786, t10532, t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836);
        let (t51564, t51572, t51576, t51578) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3047(t136, t2457, t2710, t4469, t2722, t50474, t2782, t39597, t14586, t10529, t10115, t1576);
    (t51537, t51541, t51544, t51546, t51548, t51550, t51553, t51560, t51564, t51572, t51576, t51578)
}
