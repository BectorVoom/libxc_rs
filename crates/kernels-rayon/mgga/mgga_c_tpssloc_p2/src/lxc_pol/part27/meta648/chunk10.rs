//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2248/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2248(t23384: f64, t25802: f64, t23587: f64, t7560: f64, t25410: f64, t1052: f64, t14548: f64, t23341: f64, t23346: f64, t23394: f64, t25436: f64, t25797: f64, t3016: f64, t3174: f64, t3206: f64, t4557: f64, t6687: f64, t6704: f64, t7561: f64, t7624: f64, t83435: f64, t83441: f64, t83444: f64, t89349: f64, t986: f64) -> f64 {
    let t89630 = 0.18277045187202515961e-2_f64 * t23384 * t25802;
    let t89648 = t7560 * t23587;
    let t89653 = 0.54831135561607547884e-2_f64 * t23384 * t25410;
    let t89658 = -0.27415567780803773942e-2_f64 * t83435 - 0.48738787165873375897e-2_f64 * t83441 - 0.14621636149762012769e-1_f64 * t23346 * t25802 + t89630 + 0.16449340668482264365e-1_f64 * t6687 * t6704 * t23394 * t14548 + 2.0_f64 * t1052 * t3174 * t7624 * t3206 - 0.82246703342411321825e-2_f64 * t6687 * t3016 * t7561 - 0.16449340668482264365e-1_f64 * t6687 * t89349 * t25797 - 0.36554090374405031922e-2_f64 * t83444 - 6.0_f64 * t4557 * t23341 + 0.16449340668482264365e-1_f64 * t6687 * t986 * t89648 - t89653 + 0.43864908449286038306e-1_f64 * t23346 * t25410 - 0.14621636149762012769e-1_f64 * t23346 * t25436;
    t89658
}
