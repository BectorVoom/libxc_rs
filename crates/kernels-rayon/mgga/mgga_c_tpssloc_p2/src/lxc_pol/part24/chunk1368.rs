//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1368/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1368(t6787: f64, t82573: f64, t23384: f64, t23687: f64, t23658: f64, t23665: f64, t23494: f64, t6743: f64, t23547: f64, t11023: f64, t11027: f64, t23346: f64, t23601: f64, t23610: f64, t23614: f64, t23620: f64, t23650: f64, t23670: f64, t23677: f64, t23678: f64, t6687: f64, t6784: f64, t6797: f64, t6799: f64, t6800: f64, t6801: f64, t82402: f64, t82562: f64, t82564: f64, t82566: f64, t884: f64, t986: f64) -> f64 {
    let t82574 = t82573 * t6787;
    let t82576 = t23384 * t23687;
    let t82590 = t23665 * t23658;
    let t82592 = t23494 * t6743;
    let t82596 = t23547 * t6743;
    let t82603 = 0.27415567780803773942e-2_f64 * t82562 + 0.36554090374405031922e-2_f64 * t82564 + 0.82246703342411321826e-2_f64 * t6687 * t6784 * t82566 * t884 - 0.13159472534785811492e0_f64 * t23670 * t23610 - 0.14621636149762012769e-1_f64 * t82574 + 0.54831135561607547883e-2_f64 * t82576 + 0.65797362673929057459e-1_f64 * t23346 * t23650 + 0.82246703342411321825e-2_f64 * t6797 * t6799 * t11027 * t6800 + 0.49348022005446793095e-1_f64 * t23601 * t23677 * t11023 * t23678 + 0.43864908449286038307e-1_f64 * t82402 * t23614 - 0.16449340668482264365e-1_f64 * t82590 - 0.24674011002723396548e-1_f64 * t6797 * t82592 * t6801 - 0.24674011002723396548e-1_f64 * t6797 * t82596 * t6801 - 0.24674011002723396548e-1_f64 * t6687 * t986 * t23620;
    t82603
}
