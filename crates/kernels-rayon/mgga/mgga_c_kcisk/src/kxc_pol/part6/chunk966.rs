//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 966/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk966(t29586: f64, t29589: f64, t29591: f64, t29595: f64, t29598: f64, t29601: f64, t29603: f64, t29607: f64, t29609: f64, t29611: f64, t29614: f64, t29618: f64, t29620: f64, t29622: f64, t29624: f64) -> f64 {
    let t30115 = -0.101171875e-1_f64 * t29586 - 0.15e1_f64 * t29589 + 0.32375000000000000001e0_f64 * t29591 + 0.27777777777777777777e-1_f64 * t29595 + 0.13489583333333333333e-1_f64 * t29598 - 0.161875e0_f64 * t29601 + 0.1875e0_f64 * t29603 - 0.5625e0_f64 * t29607 - 0.32375000000000000001e0_f64 * t29609 - 0.40468749999999999999e-1_f64 * t29611 + 0.16666666666666666666e0_f64 * t29614 + 0.60703125e-1_f64 * t29618 - 1.0_f64 * t29620 - 0.13489583333333333333e-1_f64 * t29622 + 0.15e1_f64 * t29624;
    t30115
}
