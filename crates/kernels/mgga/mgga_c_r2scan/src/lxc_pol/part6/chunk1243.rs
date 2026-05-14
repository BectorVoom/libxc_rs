//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1243/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1243<F: Float>(t5019: F, t4841: F, t4844: F, t4872: F, t5038: F, t4886: F, t4896: F, t18767: F, t18771: F, t18774: F, t18777: F, t4695: F, t4883: F, t4894: F, t4899: F, t6944: F, t8307: F) -> (F, F, F, F, F, F) {
    let t23244 = 0.22787578869697033845e-2 * t5019;
    let t23245 = 24.0 * t4841;
    let t23249 = 24.0 * t4844;
    let t23258 = 0.2069040516770936012e4 * t4872;
    let t23263 = 0.4155806185363551302e3 * t5038;
    let t23270 = 480.0 * t4886;
    let t23272 = 192.0 * t4896;
    let t23273 = 9.0 * t4695 + t18767 + 6.0 * t6944 + 3.0 * t8307 + 180.0 * t4883 + t23270 - t18771 - 24.0 * t4894 + t18774 - t23272 - t4899 + t18777;
    (t23244, t23245, t23249, t23258, t23263, t23273)
}
