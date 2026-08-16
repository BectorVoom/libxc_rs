//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3311/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3311(t1868: f64, t5778: f64, t22809: f64, t566: f64, t1353: f64, t1448: f64, t1450: f64, t198: f64, t22813: f64, t4139: f64, t47113: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64, t5536: f64, t5542: f64, t85987: f64, t85989: f64, t85990: f64) -> f64 {
    let t86815 = t1868 * t5778;
    let t86819 = t566 * t22809;
    let t86823 = 6.0_f64 * t1448 * t1450 * t198 * t22813 + 6.0_f64 * t1353 * t5536 * t86819 - 18.0_f64 * t4139 * t5542 * t86815 + t47113 + t47116 - t47118 + t47122 + t47124 - t85987 + t85989 + t85990;
    t86823
}
