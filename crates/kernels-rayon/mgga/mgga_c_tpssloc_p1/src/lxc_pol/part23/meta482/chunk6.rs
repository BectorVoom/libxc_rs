//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1457/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457(t44275: f64, t63361: f64, t78057: f64, t78084: f64, t78087: f64, t78090: f64, t78093: f64, t78095: f64, t78097: f64, t78100: f64, t78103: f64, t78105: f64, t78107: f64, t78109: f64) -> f64 {
    let t78853 = -0.13892666666666666667e0_f64 * t78084 - 0.125034e1_f64 * t78087 + 0.83356e0_f64 * t78090 + 0.375102e1_f64 * t78093 + 0.3529725e1_f64 * t78095 + t44275 + 0.94674375e0_f64 * t78097 + 0.27785333333333333334e0_f64 * t78100 + 0.27545333333333333333e1_f64 * t63361 + 0.1262325e1_f64 * t78103 - 0.705945e1_f64 * t78105 + 0.158837625e2_f64 * t78107 - 0.94674375e0_f64 * t78109 - 0.123954e2_f64 * t78057;
    t78853
}
