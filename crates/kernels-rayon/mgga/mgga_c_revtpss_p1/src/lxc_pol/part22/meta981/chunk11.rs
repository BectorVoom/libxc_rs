//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3322/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3322(t40945: f64, t40948: f64, t40952: f64, t40954: f64, t40956: f64, t40958: f64, t4514: f64, t51684: f64, t51686: f64, t51688: f64, t51696: f64, t51700: f64, t51703: f64, t51708: f64, t62868: f64, t837: f64) -> f64 {
    let t63041 = -0.29268663035268940438e-1_f64 * t51684 + 0.34146773541147097178e-1_f64 * t51686 + 0.2601984143835408805e-2_f64 * t51688 - 0.92526556154787137112e-2_f64 * t40945 - 0.13009920719177044025e-1_f64 * t40948 + 0.11565819519348392139e-2_f64 * t40952 + 0.13009920719177044025e-2_f64 * t40954 + 0.14634331517634470219e-1_f64 * t40956 - 0.26341796731742046394e1_f64 * t4514 * t62868 * t837 + 0.11708928647259339623e0_f64 * t51696 + 0.10975748638225852664e-1_f64 * t51700 - 0.34146773541147097178e-1_f64 * t40958 - 0.29268663035268940438e-1_f64 * t51703 + 0.10975748638225852664e-1_f64 * t51708;
    t63041
}
