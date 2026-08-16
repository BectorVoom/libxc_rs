//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 846/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk846(t31366: f64, t6555: f64, t6552: f64, t6572: f64, t1880: f64, t6547: f64, t8557: f64, t2047: f64, t234: f64, t776: f64, t6637: f64, t794: f64, t8556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31367 = t31366 * t6555;
    let t31368 = t6552 * t31367;
    let t31370 = t31366 * t6572;
    let t31371 = t1880 * t31370;
    let t31374 = t6547 * t8557;
    let t31375 = 0.19190897446562641759e-1_f64 * t31374;
    let t31376 = t234 * t2047;
    let t31377 = t31376 * t776;
    let t31378 = t6637 * t31377;
    let t31379 = t6552 * t31378;
    let t31381 = t794 * t8556;
    (t31367, t31368, t31370, t31371, t31375, t31376, t31377, t31378, t31379, t31381)
}
