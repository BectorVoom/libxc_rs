//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3239/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3239<F: Float>(t11007: F, t252: F, t2782: F, t6048: F, t886: F, t14481: F, t1569: F, t2771: F, t40970: F, t40978: F, t40986: F, t40988: F, t41078: F, t50214: F, t50218: F, t50220: F, t50222: F, t50227: F, t50232: F, t50236: F, t61397: F, t61400: F, t61403: F, t61407: F, t61411: F, t865: F) -> F {
    let t61419 = t2782 * t252 * t11007 * t6048 * t886;
    let t61422 = t2782 * t1569 * t14481;
    let t61429 = -F::cast_from(0.52039682876708176102e-2_f64) * t40970 - F::cast_from(0.39274398764404314548e-3_f64) * t40978 - F::cast_from(0.92526556154787137113e-2_f64) * t50214 - F::cast_from(0.29268663035268940438e-1_f64) * t50218 - F::cast_from(0.52039682876708176102e-1_f64) * t50220 - F::cast_from(0.29268663035268940438e-1_f64) * t50222 - F::cast_from(0.11708928647259339623e0_f64) * t50227 - F::cast_from(0.13009920719177044025e-2_f64) * t61397 + F::cast_from(0.13009920719177044025e-2_f64) * t61400 - F::cast_from(0.21951497276451705328e-1_f64) * t61403 + F::cast_from(0.23131639038696784277e-2_f64) * t61407 - F::cast_from(0.11565819519348392139e-2_f64) * t61411 - F::cast_from(0.10975748638225852664e-1_f64) * t50232 - F::cast_from(0.2601984143835408805e-2_f64) * t50236 - F::cast_from(0.73171657588172351096e-2_f64) * t40986 + F::cast_from(0.65854491829355115984e-1_f64) * t61419 - F::cast_from(0.43902994552903410656e-1_f64) * t61422 - F::cast_from(0.34146773541147097178e-1_f64) * t40988 + F::cast_from(0.15805078039045227836e2_f64) * t865 * t41078 * t6048 * t2771;
    t61429
}
