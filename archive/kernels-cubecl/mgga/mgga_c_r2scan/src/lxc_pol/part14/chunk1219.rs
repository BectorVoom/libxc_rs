//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1219/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1219<F: Float>(t39816: F, t39823: F, t39825: F, t39827: F, t39830: F, t39832: F, t39835: F, t38568: F, t39814: F, t39818: F, t39821: F, t39846: F) -> (F, F) {
    let t41570 = F::cast_from(0.11902492299418487743e0_f64) * t39816;
    let t41573 = F::cast_from(0.95219938395347901946e-2_f64) * t39823;
    let t41574 = F::cast_from(0.28565981518604370584e-1_f64) * t39825;
    let t41575 = F::cast_from(0.95219938395347901946e-2_f64) * t39827;
    let t41576 = F::cast_from(0.95219938395347901946e-2_f64) * t39830;
    let t41577 = F::cast_from(0.28565981518604370584e-1_f64) * t39832;
    let t41578 = F::cast_from(0.93149212406257582492e-1_f64) * t39835;
    let t41579 = -F::cast_from(0.21951497276451705328e0_f64) * t39814 + t41570 - F::cast_from(0.87327386630866483588e-2_f64) * t39818 + F::cast_from(0.17336443480108537126e0_f64) * t39821 - t41573 - t41574 - t41575 + t41576 - t41577 - t38568 + t41578;
    let t41582 = F::cast_from(0.84755945902752848174e0_f64) * t39846;
    (t41579, t41582)
}
