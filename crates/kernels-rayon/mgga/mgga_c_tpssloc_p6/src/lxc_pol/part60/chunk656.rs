//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 656/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk656(t6553: f64, t8547: f64, t1880: f64, t1911: f64, t2053: f64, t2718: f64, t1894: f64, t2047: f64, t214: f64, t235: f64, t8543: f64, t226: f64, t8359: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8548 = t6553 * t8547;
    let t8549 = t1880 * t8548;
    let t8553 = t2718 * t2053 * t1911;
    let t8556 = t1894 * t2047;
    let t8557 = t214 * t8556;
    let t8558 = t1880 * t8557;
    let t8560 = t235 * t8543;
    let t8562 = t8359 + 0.82246703342411321825e-2_f64 * t8558 + t226 * t8560;
    (t8548, t8549, t8553, t8556, t8557, t8560, t8562)
}
