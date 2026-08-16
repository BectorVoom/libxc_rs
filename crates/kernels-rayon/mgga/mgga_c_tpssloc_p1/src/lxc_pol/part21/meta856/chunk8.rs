//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3104/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104(t63323: f64, t63327: f64, t63330: f64, t63848: f64, t63853: f64, t63856: f64, t63858: f64, t63860: f64, t63862: f64, t63865: f64, t63867: f64, t63870: f64, t63873: f64, t63876: f64, t63879: f64) -> f64 {
    let t64325 = -0.412621875e-1_f64 * t63848 + 0.13418888888888888889e1_f64 * t63323 + 0.72462e1_f64 * t63327 - 0.48307999999999999999e1_f64 * t63330 + 0.16504875e0_f64 * t63853 + 0.16504875e0_f64 * t63856 + 0.82524375e-1_f64 * t63858 + 0.19419375e1_f64 * t63860 - 0.258925e1_f64 * t63862 - 0.258925e1_f64 * t63865 - 0.1294625e1_f64 * t63867 + 0.6189328125e-1_f64 * t63870 - 0.412621875e-1_f64 * t63873 - 0.485484375e1_f64 * t63876 + 0.19419375e1_f64 * t63879;
    t64325
}
