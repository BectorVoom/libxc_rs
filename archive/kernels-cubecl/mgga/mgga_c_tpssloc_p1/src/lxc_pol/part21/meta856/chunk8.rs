//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3104/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104<F: Float>(t63323: F, t63327: F, t63330: F, t63848: F, t63853: F, t63856: F, t63858: F, t63860: F, t63862: F, t63865: F, t63867: F, t63870: F, t63873: F, t63876: F, t63879: F) -> F {
    let t64325 = -F::cast_from(0.412621875e-1_f64) * t63848 + F::cast_from(0.13418888888888888889e1_f64) * t63323 + F::cast_from(0.72462e1_f64) * t63327 - F::cast_from(0.48307999999999999999e1_f64) * t63330 + F::cast_from(0.16504875e0_f64) * t63853 + F::cast_from(0.16504875e0_f64) * t63856 + F::cast_from(0.82524375e-1_f64) * t63858 + F::cast_from(0.19419375e1_f64) * t63860 - F::cast_from(0.258925e1_f64) * t63862 - F::cast_from(0.258925e1_f64) * t63865 - F::cast_from(0.1294625e1_f64) * t63867 + F::cast_from(0.6189328125e-1_f64) * t63870 - F::cast_from(0.412621875e-1_f64) * t63873 - F::cast_from(0.485484375e1_f64) * t63876 + F::cast_from(0.19419375e1_f64) * t63879;
    t64325
}
