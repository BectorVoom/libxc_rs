//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1363/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1363(t14143: f64, t14144: f64, t14141: f64, t10069: f64, t5737: f64, t10015: f64, t10020: f64, t10027: f64, t10032: f64, t10035: f64, t10041: f64, t10044: f64, t14116: f64, t14120: f64, t14126: f64, t14131: f64, t4004: f64, t5735: f64, t5745: f64, t9840: f64) -> f64 {
    let t14145 = t14143 * t14144;
    let t14146 = t14141 * t14145;
    let t14149 = t10069 * t5737;
    let t14151 = -t14116 - 0.19514881078765566038e-1_f64 * t10015 - 0.9757440539382783019e-2_f64 * t10020 + 0.19514881078765566038e-1_f64 * t10027 + 0.65049603595885220126e-3_f64 * t14120 + t14126 + t14131 + 0.13170898365871023197e1_f64 * t5745 * t5735 * t9840 + 0.39512695097613069591e1_f64 * t5745 * t5735 * t4004 + 0.14634331517634470219e-1_f64 * t10032 + t10035 - 0.54878743191129263322e-2_f64 * t10041 + 0.39029762157531132075e-1_f64 * t14146 - 0.13009920719177044025e-2_f64 * t10044 - 0.73171657588172351096e-2_f64 * t14149;
    t14151
}
