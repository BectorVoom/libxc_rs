//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1343/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1343(t40148: f64, t10565: f64, t717: f64, t39989: f64, t40126: f64, t40128: f64, t40131: f64, t40133: f64, t40137: f64, t40140: f64, t40142: f64, t40144: f64, t40146: f64) -> (f64, f64, f64) {
    let t40149 = 96.0_f64 * t40148;
    let t40150 = t717 * t10565;
    let t40151 = 4.0_f64 * t40150;
    let t40152 = -t40126 + t40128 - t40131 - t40133 - t40137 + t40140 + t40142 + t40144 + t40146 + t40149 - t39989 + t40151;
    (t40149, t40151, t40152)
}
