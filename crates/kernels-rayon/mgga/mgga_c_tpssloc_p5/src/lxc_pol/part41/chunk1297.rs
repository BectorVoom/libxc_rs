//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1297/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1297(t109: f64, t111715: f64, t111751: f64, t111772: f64, t111805: f64, t2205: f64, t671: f64, t100930: f64, t110363: f64, t111226: f64, t111246: f64, t1401: f64, t1458: f64, t16524: f64, t19534: f64, t20162: f64, t20173: f64, t2199: f64, t28893: f64, t30315: f64, t30363: f64, t30382: f64, t30385: f64, t30611: f64, t3941: f64, t4072: f64, t5371: f64, t5376: f64, t5456: f64, t5493: f64, t75795: f64, t8189: f64, t8207: f64, t8294: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t111808 = piecewise3(t110, 0.0_f64, t111715 + t111751 + t111772 + t111805);
    let t111819 = t2205 * t671;
    let t111842 = 27.0_f64 * t110363 * t5456 + 27.0_f64 * t28893 * t8189 + 0.135e2_f64 * t1401 * t111808 + 54.0_f64 * t111246 * t5376 + 54.0_f64 * t16524 * t30385 + 27.0_f64 * t100930 * t2199 + 0.135e2_f64 * t8207 * t19534 + 27.0_f64 * t111819 * t5456 + 27.0_f64 * t111226 * t1458 + 54.0_f64 * t16524 * t30382 + 27.0_f64 * t20173 * t30611 + 27.0_f64 * t30363 * t4072 + 54.0_f64 * t75795 * t8294 + 27.0_f64 * t5371 * t30315 + 0.135e2_f64 * t20162 * t8189 + 27.0_f64 * t3941 * t8189 * t5493 + 27.0_f64 * t3941 * t2199 * t19534;
    (t111808, t111842)
}
