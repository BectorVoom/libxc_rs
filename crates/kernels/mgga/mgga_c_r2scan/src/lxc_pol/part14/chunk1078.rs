//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1078/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1078<F: Float>(t39830: F, t39832: F, t39835: F, t38568: F, t39814: F, t39818: F, t39821: F, t41570: F, t41573: F, t41574: F, t41575: F, t39846: F, t39854: F, t37925: F, t37933: F, t39838: F, t39843: F, t39851: F, t39857: F, t39859: F, t39863: F, t39866: F, t39869: F) -> (F, F) {
    let t41576 = 0.95219938395347901946e-2 * t39830;
    let t41577 = 0.28565981518604370584e-1 * t39832;
    let t41578 = 0.93149212406257582492e-1 * t39835;
    let t41579 = -0.21951497276451705328e0 * t39814 + t41570 - 0.87327386630866483588e-2 * t39818 + 0.17336443480108537126e0 * t39821 - t41573 - t41574 - t41575 + t41576 - t41577 - t38568 + t41578;
    let t41582 = 0.84755945902752848174e0 * t39846;
    let t41584 = 0.13869154784086829701e1 * t39854;
    let t41592 = 0.87327386630866483588e-2 * t39838 - 0.26198215989259945076e-1 * t39843 - t41582 - 0.13170898365871023197e1 * t39851 - t41584 - 0.55476619136347318806e1 * t39857 + 0.5200933044032561138e0 * t39859 + 0.12805040077930161442e0 * t37925 - 0.85366933852867742946e0 * t37933 + 0.34672886960217074252e0 * t39863 + 0.34672886960217074252e0 * t39866 + 0.5200933044032561138e0 * t39869;
    (t41579, t41592)
}
