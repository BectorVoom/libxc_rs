//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 971/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk971(t2367: f64, t4616: f64, t876: f64, t2402: f64, t794: f64, t2134: f64, t27: f64, t4895: f64, t649: f64, t6355: f64, t7810: f64, t2344: f64, t35674: f64) -> (f64, f64, f64, f64, f64) {
    let t40596 = t4616 * t2367;
    let t40597 = t40596 * t876;
    let t40602 = t2402 * t794;
    let t40607 = t2134 * t27 * t649 * t4895;
    let t40610 = t6355 * t7810;
    let t40614 = t35674 * t2344;
    (t40597, t40602, t40607, t40610, t40614)
}
