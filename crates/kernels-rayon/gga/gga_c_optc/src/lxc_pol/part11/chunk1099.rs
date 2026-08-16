//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1099/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1099(t5021: f64, t7274: f64, t913: f64, t25622: f64, t2721: f64, t5025: f64, t2742: f64, t2778: f64, t5016: f64, t2693: f64, t4983: f64, t10594: f64, t4054: f64) -> (f64, f64, f64, f64, f64) {
    let t42991 = t913 * t7274 * t5021;
    let t43003 = t2721 * t25622 * t5025;
    let t43112 = t2778 * t2742 * t5016;
    let t43210 = t4983 * t2693;
    let t43260 = t4054 * t10594;
    (t42991, t43003, t43112, t43210, t43260)
}
