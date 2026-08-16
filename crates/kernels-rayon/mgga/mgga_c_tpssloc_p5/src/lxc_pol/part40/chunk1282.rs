//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1282/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1282(t3: f64, t30465: f64, t1458: f64, t8230: f64, t2180: f64, t5493: f64, t1401: f64, t16524: f64, t20162: f64, t28893: f64, t29996: f64, t30231: f64, t30424: f64, t3941: f64, t5371: f64, t5456: f64, t577: f64, t8161: f64, t8251: f64) -> (f64, f64, f64, f64) {
    let t30466 = t3 * t30465;
    let t30492 = t8230 * t1458;
    let t30495 = t2180 * t5493;
    let t30500 = 0.45e1_f64 * t30465 * t577 + 27.0_f64 * t30231 * t1458 + 27.0_f64 * t29996 * t5456 + 0.135e2_f64 * t8161 * t5493 + 0.135e2_f64 * t20162 * t2180 + 54.0_f64 * t16524 * t8251 + 27.0_f64 * t5371 * t8230 + 27.0_f64 * t28893 * t2180 + 54.0_f64 * t3941 * t30492 + 27.0_f64 * t3941 * t30495 + 0.135e2_f64 * t1401 * t30424;
    (t30466, t30492, t30495, t30500)
}
