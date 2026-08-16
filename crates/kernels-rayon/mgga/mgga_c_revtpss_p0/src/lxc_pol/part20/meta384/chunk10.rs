//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1412/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1412(t10627: f64, t198: f64, t40067: f64, t40072: f64, t40155: f64, t40157: f64, t40160: f64, t40163: f64, t40167: f64, t40171: f64, t40173: f64, t40175: f64, t40179: f64, t40184: f64, t40187: f64, t890: f64, t892: f64) -> f64 {
    let t41191 = 24.0_f64 * t10627 * t198 * t890 * t892 + t40067 - t40072 + t40155 - t40157 + t40160 + t40163 + t40167 - t40171 - t40173 + t40175 + t40179 - t40184 + t40187;
    t41191
}
