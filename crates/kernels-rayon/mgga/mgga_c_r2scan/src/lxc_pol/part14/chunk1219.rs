//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1219/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1219(t39816: f64, t39823: f64, t39825: f64, t39827: f64, t39830: f64, t39832: f64, t39835: f64, t38568: f64, t39814: f64, t39818: f64, t39821: f64, t39846: f64) -> (f64, f64) {
    let t41570 = 0.11902492299418487743e0_f64 * t39816;
    let t41573 = 0.95219938395347901946e-2_f64 * t39823;
    let t41574 = 0.28565981518604370584e-1_f64 * t39825;
    let t41575 = 0.95219938395347901946e-2_f64 * t39827;
    let t41576 = 0.95219938395347901946e-2_f64 * t39830;
    let t41577 = 0.28565981518604370584e-1_f64 * t39832;
    let t41578 = 0.93149212406257582492e-1_f64 * t39835;
    let t41579 = -0.21951497276451705328e0_f64 * t39814 + t41570 - 0.87327386630866483588e-2_f64 * t39818 + 0.17336443480108537126e0_f64 * t39821 - t41573 - t41574 - t41575 + t41576 - t41577 - t38568 + t41578;
    let t41582 = 0.84755945902752848174e0_f64 * t39846;
    (t41579, t41582)
}
