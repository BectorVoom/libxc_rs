//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 702/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk702<F: Float>(t15606: F, t15609: F, t15612: F, t15891: F, t15894: F, t15604: F, t15617: F, t15621: F, t15628: F, t15888: F, t15897: F, t15899: F, t15904: F, t15907: F, t15910: F, t15915: F, t15919: F, t15922: F, t15925: F, t15929: F, t15934: F, t15938: F) -> (F, F) {
    let t16336 = 2.0 / 27.0 * t15606;
    let t16337 = 2.0 / 9.0 * t15609;
    let t16338 = t15612 / 9.0;
    let t16342 = t15891 / 3.0;
    let t16343 = 2.0 / 3.0 * t15894;
    let t16345 = -6.0 * t15604 + t16336 - t16337 + t16338 + 2.0 * t15617 + 4.0 * t15621 - t15628 / 3.0 - t15888 + t16342 - t16343 - 8.0 / 9.0 * t15897;
    let t16346 = 2.0 / 9.0 * t15899;
    let t16357 = -t16346 + t15904 / 3.0 + 2.0 / 3.0 * t15907 - 2.0 / 9.0 * t15910 - 2.0 / 3.0 * t15915 - 2.0 / 3.0 * t15919 - 2.0 * t15922 + 8.0 / 3.0 * t15925 + t15929 / 3.0 + 2.0 / 3.0 * t15934 + 4.0 / 3.0 * t15938;
    (t16345, t16357)
}
