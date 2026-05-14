//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1341/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1341<F: Float>(t4189: F, t5886: F, t4306: F, t5606: F, t109279: F, t6373: F, t1493: F, t6309: F, t109226: F, t9836: F, t113511: F, t113513: F, t113515: F, t113517: F, t113519: F, t113521: F) -> (F, F, F, F, F, F) {
    let t113523 = t5886 * t4189;
    let t113525 = t5606 * t4306;
    let t113527 = t109279 * t6373;
    let t113529 = t6309 * t1493;
    let t113531 = t109226 * t9836;
    let t113533 = 11.0 / 27.0 * t113511 - t113513 / 128.0 - t113515 / 12.0 + t113517 / 9.0 + 2.0 / 9.0 * t113519 - t113521 / 12.0 + t113523 / 128.0 - t113525 / 288.0 + t113527 / 48.0 - t113529 / 3.0 - 11.0 / 18.0 * t113531;
    (t113523, t113525, t113527, t113529, t113531, t113533)
}
