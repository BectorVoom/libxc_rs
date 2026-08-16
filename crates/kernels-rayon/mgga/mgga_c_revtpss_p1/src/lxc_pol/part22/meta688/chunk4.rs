//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2683/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2683(t10032: f64, t10035: f64, t10044: f64, t1399: f64, t14116: f64, t14120: f64, t14126: f64, t14131: f64, t14146: f64, t14149: f64, t14158: f64, t14161: f64, t14166: f64, t21981: f64, t21990: f64, t4118: f64, t5735: f64, t5745: f64, t5755: f64, t6844: f64, t820: f64) -> f64 {
    let t21998 = -t14116 + 0.13009920719177044025e-2_f64 * t14120 + t14126 + t14131 - 0.13170898365871023197e1_f64 * t5755 * t21981 * t1399 + 0.73171657588172351096e-2_f64 * t10032 + t10035 + 0.39029762157531132076e-1_f64 * t14146 - 0.65049603595885220126e-3_f64 * t10044 - 0.14634331517634470219e-1_f64 * t14149 + t14158 + 0.23131639038696784278e-2_f64 * t14161 + 0.26341796731742046394e1_f64 * t5745 * t5735 * t21990 + 0.14634331517634470219e-1_f64 * t14166 - 0.65854491829355115987e0_f64 * t820 * t4118 * t6844;
    t21998
}
