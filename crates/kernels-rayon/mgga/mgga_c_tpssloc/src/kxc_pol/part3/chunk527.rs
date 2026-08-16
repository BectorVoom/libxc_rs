//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 527/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk527(t2374: f64, t2375: f64, t200: f64, t262: f64, t776: f64) -> (f64, f64, f64) {
    let t2377 = 0.10843581300301739842e-1_f64 * t2374 * t2375;
    let t2378 = t200 * t262;
    let t2379 = t776 * t776;
    (t2377, t2378, t2379)
}
