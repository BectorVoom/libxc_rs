//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1280/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1280(t30328: f64, t30347: f64, t3: f64, t112: f64, t8283: f64, t1458: f64, t8189: f64, t2199: f64, t4072: f64, t671: f64, t8273: f64, t12524: f64, t1401: f64, t16521: f64, t16524: f64, t20173: f64, t30109: f64, t30112: f64, t30315: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64, t8207: f64, t8212: f64, t8294: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30349 = 2.0_f64 * t30328 + 2.0_f64 * t30347;
    let t30350 = t3 * t30349;
    let t30363 = t8283 * t112;
    let t30382 = t8189 * t1458;
    let t30385 = t2199 * t4072;
    let t30390 = t8273 * t671;
    let t30395 = 0.45e1_f64 * t30349 * t577 + 0.135e2_f64 * t30363 * t671 + 0.135e2_f64 * t30109 * t1458 + 27.0_f64 * t30112 * t5376 + 0.135e2_f64 * t8207 * t4072 + 0.135e2_f64 * t16521 * t2199 + 27.0_f64 * t16524 * t8212 + 0.135e2_f64 * t5371 * t8189 + 27.0_f64 * t12524 * t8294 + 27.0_f64 * t20173 * t8294 + 27.0_f64 * t3941 * t30382 + 27.0_f64 * t3941 * t30385 + 0.135e2_f64 * t3938 * t8273 + 27.0_f64 * t3941 * t30390 + 0.135e2_f64 * t1401 * t30315;
    (t30349, t30350, t30363, t30382, t30385, t30390, t30395)
}
