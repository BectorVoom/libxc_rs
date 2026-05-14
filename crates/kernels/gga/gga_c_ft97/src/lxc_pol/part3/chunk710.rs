//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 710/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk710<F: Float>(t15899: F, t15904: F, t15907: F, t15910: F, t15915: F, t15919: F, t15922: F, t15925: F, t15929: F, t15934: F, t15938: F, t11027: F, t11638: F, t11646: F, t11647: F, t11648: F, t15942: F, t15945: F, t15948: F, t15953: F, t15957: F, t15961: F) -> (F, F) {
    let t16515 = -2.0 / 27.0 * t15899 + t15904 / 9.0 + 2.0 / 9.0 * t15907 - 2.0 / 27.0 * t15910 - 2.0 / 9.0 * t15915 - 2.0 / 9.0 * t15919 - 2.0 / 3.0 * t15922 + 8.0 / 9.0 * t15925 + t15929 / 9.0 + 2.0 / 9.0 * t15934 + 4.0 / 9.0 * t15938;
    let t16523 = 2.0 / 27.0 * t15942 + 4.0 / 9.0 * t15945 - 10.0 / 81.0 * t15948 - t11638 - t11646 - t11647 + t11648 - 4.0 / 9.0 * t15953 + 4.0 / 27.0 * t15957 - 4.0 / 9.0 * t15961 - t11027;
    (t16515, t16523)
}
