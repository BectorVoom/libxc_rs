//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1424/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1424(t44053: f64, t63361: f64, t78057: f64, t78084: f64, t78087: f64, t78090: f64, t78093: f64, t78095: f64, t78097: f64, t78100: f64, t78103: f64, t78105: f64, t78107: f64, t78109: f64) -> f64 {
    let t78191 = -0.10954222222222222222e0_f64 * t78084 - 0.98587999999999999999e0_f64 * t78087 + 0.65725333333333333332e0_f64 * t78090 + 0.295764e1_f64 * t78093 + 0.1898925e1_f64 * t78095 + t44053 + 0.46074375e0_f64 * t78097 + 0.21908444444444444444e0_f64 * t78100 + 0.15944888888888888889e1_f64 * t63361 + 0.614325e0_f64 * t78103 - 0.379785e1_f64 * t78105 + 0.85451625e1_f64 * t78107 - 0.46074375e0_f64 * t78109 - 0.71752000000000000002e1_f64 * t78057;
    t78191
}
