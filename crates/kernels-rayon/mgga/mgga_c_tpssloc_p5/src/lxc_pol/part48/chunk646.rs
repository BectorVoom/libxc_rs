//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 646/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk646(t1894: f64, t2047: f64, t214: f64, t1880: f64, t235: f64, t8543: f64, t226: f64, t8359: f64, t858: f64) -> (f64, f64, f64, f64, f64) {
    let t8556 = t1894 * t2047;
    let t8557 = t214 * t8556;
    let t8558 = t1880 * t8557;
    let t8560 = t235 * t8543;
    let t8562 = t8359 + 0.82246703342411321825e-2_f64 * t8558 + t226 * t8560;
    let t8563 = t858 * t8562;
    (t8556, t8557, t8560, t8562, t8563)
}
