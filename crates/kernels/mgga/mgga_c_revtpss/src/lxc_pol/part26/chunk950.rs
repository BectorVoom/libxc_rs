//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 950/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk950<F: Float>(t2047: F, t25163: F, t6963: F, t7349: F, t10301: F, t7342: F, t6954: F, t239: F, t72: F, t1927: F, t1923: F, t2048: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26170: F, t26172: F, t26175: F, t26180: F, t6960: F, t7343: F, t7352: F) -> (F, F, F, F, F) {
    let t26182 = t2047 * t25163;
    let t26185 = t6963 * t7349;
    let t26187 = t10301 * t7342;
    let t26190 = t6954 * t7349;
    let t26204 = t239 * t72;
    let t26205 = t26204 * t1927;
    let t26207 = 88.0 / 27.0 * t1923 * t26205;
    let t26208 = t25150 * t2048 / 3.0 + 2.0 / 3.0 * t6954 * t7352 - 16.0 / 9.0 * t26170 + t1923 * t26172 / 3.0 + 10.0 * t26175 * t25159 + 80.0 / 9.0 * t26180 + 20.0 / 3.0 * t25162 * t26182 + 32.0 / 9.0 * t26185 - 10.0 / 3.0 * t26187 * t6960 - 16.0 / 9.0 * t26190 - 4.0 / 3.0 * t25102 * t2048 - 10.0 / 3.0 * t7343 * t25110 - 5.0 / 3.0 * t7343 * t25114 - 2.0 / 3.0 * t25117 * t2048 - 2.0 / 3.0 * t25120 * t2048 - 4.0 / 3.0 * t6963 * t7352 + t26207;
    (t26182, t26187, t26204, t26205, t26208)
}
