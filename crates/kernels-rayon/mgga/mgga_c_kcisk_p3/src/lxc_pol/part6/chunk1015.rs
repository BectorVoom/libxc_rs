//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1015/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1015(t30655: f64, t334: f64, t2093: f64, t25663: f64, t5715: f64, t7786: f64, t19476: f64, t7789: f64, t1201: f64, t30553: f64, t30557: f64, t30561: f64, t30564: f64, t30567: f64, t30641: f64, t30644: f64, t45: f64) -> (f64, f64, f64, f64, f64) {
    let t30656 = t30655 * t334;
    let t30660 = 3.0_f64 * t25663 * t2093;
    let t30662 = 3.0_f64 * t5715 * t7786;
    let t30664 = 0.48245472966453314466e2_f64 * t19476 * t7789;
    let t30665 = -0.35089340384731224426e1_f64 * t1201 * t30553 - t30557 + t30561 - t30564 + t30567 + t30641 + t30644 + 0.19751789702565206229e-1_f64 * t45 * t30656 + t30660 + t30662 + t30664;
    (t30656, t30660, t30662, t30664, t30665)
}
