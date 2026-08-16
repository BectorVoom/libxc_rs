//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1196/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1196(t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t40018: f64, t6353: f64, t12189: f64, t6358: f64, t19767: f64, t40409: f64) -> (f64, f64, f64, f64, f64) {
    let t56463 = t212 * t6330;
    let t56465 = t2586 * t40353 * t56463;
    let t56467 = t212 * t6347;
    let t56469 = t2586 * t12225 * t56467;
    let t56484 = t40018 * t6353;
    let t56491 = t12189 * t6358;
    let t56535 = t40409 * t19767;
    (t56465, t56469, t56484, t56491, t56535)
}
