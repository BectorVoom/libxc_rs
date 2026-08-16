//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1346/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346(t40178: f64, t40067: f64, t40072: f64, t40155: f64, t40157: f64, t40160: f64, t40163: f64, t40167: f64, t40171: f64, t40173: f64, t40175: f64, t39909: f64, t738: f64, t745: f64) -> (f64, f64, f64) {
    let t40179 = 144.0_f64 * t40178;
    let t40180 = t40155 - t40157 + t40067 - t40072 + t40160 + t40163 + t40167 - t40171 - t40173 + t40175 + t40179;
    let t40182 = t738 * t39909 * t745;
    (t40179, t40180, t40182)
}
