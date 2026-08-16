//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2177/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2177(t20038: f64, t225: f64, t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t118: f64, t19631: f64, t3739: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56434 = t20038 * t225;
    let t56463 = t212 * t6330;
    let t56465 = t2586 * t40353 * t56463;
    let t56467 = t212 * t6347;
    let t56469 = t2586 * t12225 * t56467;
    let t56482 = t3739 * t118 * t794 * t19631;
    (t56434, t56463, t56465, t56467, t56469, t56482)
}
