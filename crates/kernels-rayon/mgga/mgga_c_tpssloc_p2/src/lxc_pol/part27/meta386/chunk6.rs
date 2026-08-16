//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1586/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1586(t3640: f64, t5091: f64, t3415: f64, t4869: f64, t1654: f64, t2394: f64) -> (f64, f64, f64) {
    let t14696 = t5091 * t3640;
    let t14701 = 0.11696447245269292414e1_f64 * t4869 * t3415;
    let t14702 = t2394 * t1654;
    (t14696, t14701, t14702)
}
