//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1343/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1343<F: Float>(t20922: F, t32244: F, t2282: F, t32308: F, t4170: F, t32226: F, t6394: F, t113320: F, t113322: F, t113324: F, t113347: F, t113537: F, t15094: F, t1620: F, t21345: F, t22052: F, t22151: F, t2347: F, t2748: F, t32336: F, t32523: F, t32526: F, t33708: F, t33743: F, t33745: F, t41849: F, t4535: F, t4536: F, t4565: F, t6638: F, t9571: F, t9882: F, t9891: F) -> (F, F, F, F) {
    let t113539 = 2.0 * t20922 * t32244;
    let t113543 = 2.0 * t4170 * t32308 * t2282;
    let t113547 = 2.0 * t32226 * t6394;
    let t113551 = -12.0 * t15094 * t1620 * t33708 - 6.0 * t15094 * t4536 * t9891 + 4.0 * t1620 * t33743 * t4535 + 2.0 * t22151 * t2748 * t4535 + 2.0 * t2347 * t32336 * t4535 + 24.0 * t41849 * t4536 * t9882 + 4.0 * t4535 * t6638 * t9571 + 4.0 * t21345 * t32526 + 2.0 * t22052 * t32523 - t33745 * t4565 - t113320 + t113322 - t113324 - t113347 + t113537 - t113539 - t113543 + t113547;
    (t113539, t113543, t113547, t113551)
}
