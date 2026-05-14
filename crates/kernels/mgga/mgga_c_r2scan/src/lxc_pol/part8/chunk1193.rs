//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1193/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1193<F: Float>(t5019: F, t4841: F, t4844: F, t4872: F, t5038: F, t4886: F, t4896: F, t4980: F, t4797: F, t5021: F, t7035: F, t7094: F, t7096: F, t7125: F, t18789: F, t18791: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23244 = 0.22787578869697033845e-2 * t5019;
    let t23245 = 24.0 * t4841;
    let t23249 = 24.0 * t4844;
    let t23258 = 0.2069040516770936012e4 * t4872;
    let t23263 = 0.4155806185363551302e3 * t5038;
    let t23270 = 480.0 * t4886;
    let t23272 = 192.0 * t4896;
    let t23281 = 48.0 * t4980;
    let t23283 = 0.34367190188705947438e1 * t4797;
    let t23296 = 24.0 * t5021;
    let t23297 = 0.10986868383603927032e-2 * t7035;
    let t23306 = 24.0 * t7094;
    let t23307 = 24.0 * t7096;
    let t23312 = 6.0 * t7125;
    let t23320 = 144.0 * t18789;
    let t23321 = 240.0 * t18791;
    (t23244, t23245, t23249, t23258, t23263, t23270, t23272, t23281, t23283, t23296, t23297, t23306, t23307, t23312, t23320, t23321)
}
