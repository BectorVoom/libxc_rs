//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3322/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3322<F: Float>(t40945: F, t40948: F, t40952: F, t40954: F, t40956: F, t40958: F, t4514: F, t51684: F, t51686: F, t51688: F, t51696: F, t51700: F, t51703: F, t51708: F, t62868: F, t837: F) -> F {
    let t63041 = -F::cast_from(0.29268663035268940438e-1_f64) * t51684 + F::cast_from(0.34146773541147097178e-1_f64) * t51686 + F::cast_from(0.2601984143835408805e-2_f64) * t51688 - F::cast_from(0.92526556154787137112e-2_f64) * t40945 - F::cast_from(0.13009920719177044025e-1_f64) * t40948 + F::cast_from(0.11565819519348392139e-2_f64) * t40952 + F::cast_from(0.13009920719177044025e-2_f64) * t40954 + F::cast_from(0.14634331517634470219e-1_f64) * t40956 - F::cast_from(0.26341796731742046394e1_f64) * t4514 * t62868 * t837 + F::cast_from(0.11708928647259339623e0_f64) * t51696 + F::cast_from(0.10975748638225852664e-1_f64) * t51700 - F::cast_from(0.34146773541147097178e-1_f64) * t40958 - F::cast_from(0.29268663035268940438e-1_f64) * t51703 + F::cast_from(0.10975748638225852664e-1_f64) * t51708;
    t63041
}
