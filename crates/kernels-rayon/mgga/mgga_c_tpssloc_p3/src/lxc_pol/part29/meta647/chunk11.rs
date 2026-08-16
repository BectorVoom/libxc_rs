//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2153/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2153(t23270: f64, t2379: f64, t25039: f64, t87642: f64, t1880: f64, t23218: f64, t25224: f64, t6562: f64, t6572: f64, t86893: f64, t23171: f64, t23228: f64, t7488: f64) -> (f64, f64, f64, f64) {
    let t87765 = t87642 * t23270 * t25039 * t2379;
    let t87773 = t1880 * t25224 * t23218;
    let t87776 = t6562 * t86893 * t6572;
    let t87777 = 0.82246703342411321824e-2_f64 * t87776;
    let t87779 = t23171 * t23228 * t7488;
    (t87765, t87773, t87777, t87779)
}
