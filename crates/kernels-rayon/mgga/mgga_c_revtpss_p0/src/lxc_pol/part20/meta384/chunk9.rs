//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1411/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1411(t10489: f64, t11084: f64, t14375: f64, t198: f64, t2403: f64, t2430: f64, t262: f64, t39989: f64, t40128: f64, t40131: f64, t40133: f64, t40137: f64, t40140: f64, t40142: f64, t40144: f64, t40146: f64, t40149: f64, t40151: f64, t4541: f64, t775: f64) -> f64 {
    let t41185 = 24.0_f64 * t10489 * t262 * t4541 * t775 - 18.0_f64 * t11084 * t2403 * t2430 + 36.0_f64 * t14375 * t198 * t2430 - t39989 + t40128 - t40131 - t40133 - t40137 + t40140 + t40142 + t40144 + t40146 + t40149 + t40151;
    t41185
}
