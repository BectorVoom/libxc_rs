//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1398/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398(t235: f64, t46475: f64, t239: f64, t820: f64, t2482: f64, t4000: f64, t596: f64, t72: f64, t9940: f64, t245: f64, t136: f64, t4010: f64) -> (f64, f64, f64, f64) {
    let t47201 = t46475 * t235;
    let t47203 = t820 * t47201 * t239;
    let t47215 = t2482 * t4000 * t596;
    let t47247 = t9940 * t72;
    let t47248 = t47247 * t245;
    let t47273 = t4010 * t136;
    (t47203, t47215, t47248, t47273)
}
