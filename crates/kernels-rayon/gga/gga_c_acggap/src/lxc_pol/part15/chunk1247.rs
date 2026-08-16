//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1247/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1247(t32782: f64, t32795: f64, t32796: f64, t32799: f64, t32800: f64, t32803: f64, t35560: f64, t35562: f64, t35563: f64, t37605: f64, t37606: f64, t37607: f64, t40043: f64, t40045: f64, t40047: f64, t40050: f64, t40054: f64, t40057: f64) -> f64 {
    let t41911 = t32782 - t37605 + t37606 - 0.12579236915841660828e-2_f64 * t40043 - t37607 + 35.0_f64 / 108.0_f64 * t35560 - t32795 - t32796 - 0.84046875e-1_f64 * t40045 - 0.5603125e-1_f64 * t40047 + t40050 / 4.0_f64 + t35562 + t35563 + t32799 - t32800 - t32803 - 0.68598428988911579155e-1_f64 * t40054 + 0.12862205435420921092e-1_f64 * t40057;
    t41911
}
