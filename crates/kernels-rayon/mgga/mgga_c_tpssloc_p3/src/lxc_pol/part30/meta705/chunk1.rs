//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2309/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309(t5936: f64, t6743: f64, t1022: f64, t5392: f64, t6800: f64, t23518: f64, t5928: f64, t17843: f64, t1949: f64, t23346: f64, t23604: f64, t23633: f64, t25554: f64, t28602: f64, t28610: f64, t3180: f64, t5844: f64, t6687: f64, t6805: f64, t83239: f64, t83240: f64, t83245: f64, t884: f64, t89256: f64, t89292: f64, t89294: f64, t89296: f64) -> (f64, f64) {
    let t100231 = t6743 * t5936;
    let t100236 = t5392 * t1022 * t6800;
    let t100240 = t23518 * t5928;
    let t100253 = t89256 + 2.0_f64 * t3180 * t28602 + 0.27415567780803773942e-2_f64 * t23633 * t100231 * t25554 + 0.36554090374405031923e-2_f64 * t83239 * t83240 * t100236 - 0.27415567780803773942e-2_f64 * t83245 * t100240 * t23604 * t884 - 0.82246703342411321825e-2_f64 * t6687 * t17843 * t1949 - 0.82246703342411321825e-2_f64 * t6687 * t5844 * t6805 - t89292 + t89294 - t89296 - 0.14621636149762012769e-1_f64 * t23346 * t28610;
    (t100236, t100253)
}
