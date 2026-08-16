//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1285/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1285(t3: f64, t30581: f64, t1458: f64, t8273: f64, t2199: f64, t5493: f64, t1401: f64, t16524: f64, t20162: f64, t28893: f64, t30112: f64, t30363: f64, t30534: f64, t3941: f64, t5371: f64, t5456: f64, t577: f64, t8207: f64, t8294: f64) -> (f64, f64, f64, f64) {
    let t30582 = t3 * t30581;
    let t30608 = t8273 * t1458;
    let t30611 = t2199 * t5493;
    let t30616 = 0.45e1_f64 * t30581 * t577 + 27.0_f64 * t30363 * t1458 + 27.0_f64 * t30112 * t5456 + 0.135e2_f64 * t8207 * t5493 + 0.135e2_f64 * t20162 * t2199 + 54.0_f64 * t16524 * t8294 + 27.0_f64 * t5371 * t8273 + 27.0_f64 * t28893 * t2199 + 54.0_f64 * t3941 * t30608 + 27.0_f64 * t3941 * t30611 + 0.135e2_f64 * t1401 * t30534;
    (t30582, t30608, t30611, t30616)
}
