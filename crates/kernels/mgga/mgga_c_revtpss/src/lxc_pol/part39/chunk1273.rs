//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1273/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1273<F: Float>(t15687: F, t3623: F, t3782: F, t1263: F, t1794: F, t372: F, t12712: F, t3629: F, t17301: F, t17304: F, t17308: F, t17311: F, t17333: F, t17337: F, t17339: F, t17340: F, t17342: F, t17344: F, t17347: F, t3674: F, t484: F) -> (F, F, F) {
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    let t17352 = t1263 * t1794;
    let t17353 = t372 * t17352;
    let t17354 = t12712 * t3629;
    let t17355 = t17353 * t17354;
    let t17358 = -t17301 + 0.47637797908966374413e-4 * t17304 + 0.42874018118069736972e-3 * t17308 * t3674 - 0.11433071498151929859e-2 * t17311 * t484 + 0.21437009059034868486e-3 * t17333 * t484 - t17337 + t17339 + 0.2540682555144873302e-3 * t17340 - 0.47637797908966374413e-4 * t17342 - 0.12862205435420921092e-2 * t17344 * t17347 + 0.28582678745379824648e-3 * t17351 * t17355;
    (t17350, t17353, t17358)
}
