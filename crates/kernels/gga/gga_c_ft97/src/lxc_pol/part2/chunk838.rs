//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 838/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk838<F: Float>(t15037: F, t2681: F, t824: F, t2347: F, t852: F, t3886: F, t2360: F, t1212: F, t2781: F, t2666: F, t10559: F, t10584: F, t10586: F, t10589: F, t10591: F, t10594: F, t10595: F, t10617: F, t10619: F, t13682: F, t13688: F, t15011: F, t15014: F, t15015: F, t15018: F, t15022: F, t15025: F, t15028: F, t462: F) -> (F,) {
    let t15039 = t2681 * t15037 * t824;
    let t15042 = t852 * t2347;
    let t15043 = t3886 * t824;
    let t15044 = t15042 * t15043;
    let t15047 = t852 * t2360;
    let t15048 = t15047 * t15043;
    let t15051 = t2781 * t1212;
    let t15052 = t15051 * t2666;
    let t15055 = -4.0 / 9.0 * t15011 + t15014 - 22.0 / 9.0 * t15015 - 6.0 * t462 * t15018 + 2.0 * t462 * t15022 - 4.0 / 27.0 * t15025 - t15028 - 2.0 / 9.0 * t10617 + t10559 / 3.0 - 2.0 / 3.0 * t10584 - 8.0 / 9.0 * t10595 - 8.0 / 27.0 * t10586 + t10589 / 9.0 + 2.0 / 27.0 * t10591 - 2.0 / 9.0 * t10619 + 4.0 * t462 * t15039 + 4.0 / 9.0 * t13682 * t15044 - 4.0 / 3.0 * t13688 * t15048 - 4.0 / 3.0 * t13688 * t15052 - t10594;
    (t15055,)
}
