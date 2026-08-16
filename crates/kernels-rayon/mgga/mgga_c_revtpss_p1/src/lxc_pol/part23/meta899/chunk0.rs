//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2859/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859(t1469: f64, t4401: f64, t61266: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t50874: f64, t50884: f64, t77020: f64, t77021: f64, t77023: f64, t77024: f64, t77025: f64, t77026: f64, t77027: f64, t77028: f64, t77029: f64) -> (f64, f64) {
    let t77032 = 36.0_f64 * t4401 * t61266 * t1469;
    let t77033 = t77020 + t77021 + t40067 - t40072 + t50874 + t77023 + t40167 - t40171 - t77024 - t40184 + t77025 + t77026 + t77027 + t50884 - t77028 + t77029 + t77032;
    (t77032, t77033)
}
