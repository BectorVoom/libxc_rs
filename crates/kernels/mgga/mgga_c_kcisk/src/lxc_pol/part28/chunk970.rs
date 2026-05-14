//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 970/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk970<F: Float>(t10570: F, t10649: F, t15989: F, t15992: F, t15994: F, t15996: F, t22564: F, t22567: F, t22570: F, t22573: F, t22575: F, t22578: F, t22581: F, t22583: F, t22586: F, t22589: F, t22594: F) -> (F,) {
    let t22596 = -t10649 - 4.0 / 27.0 * t10570 - 8.0 / 27.0 * t15989 + t15992 - t15994 - 4.0 / 9.0 * t15996 + 2.0 / 27.0 * t22564 - 10.0 / 27.0 * t22567 + 4.0 / 3.0 * t22570 + 8.0 / 9.0 * t22573 - 2.0 / 9.0 * t22575 - 2.0 * t22578 - 8.0 / 3.0 * t22581 + t22583 / 9.0 - 2.0 / 9.0 * t22586 + 2.0 / 3.0 * t22589 - t22594 / 3.0;
    (t22596,)
}
