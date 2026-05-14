//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 681/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk681<F: Float>(t432: F, t4462: F, t1564: F, t446: F, t15604: F, t15606: F, t15609: F, t15612: F, t15617: F, t15621: F, t15628: F, t15888: F, t15891: F, t15894: F, t15897: F, t15899: F, t15904: F, t15907: F, t15910: F, t15915: F, t15919: F, t15922: F, t15925: F) -> (F, F, F) {
    let t15927 = t4462 * t432;
    let t15928 = t1564 * t15927;
    let t15929 = t446 * t15928;
    let t15931 = -t15604 + t15606 / 81.0 - t15609 / 27.0 + t15612 / 54.0 + t15617 / 3.0 + 2.0 / 3.0 * t15621 - t15628 / 18.0 - t15888 / 6.0 + t15891 / 18.0 - t15894 / 9.0 - 4.0 / 27.0 * t15897 - t15899 / 27.0 + t15904 / 18.0 + t15907 / 9.0 - t15910 / 27.0 - t15915 / 9.0 - t15919 / 9.0 - t15922 / 3.0 + 4.0 / 9.0 * t15925 + t15929 / 18.0;
    (t15927, t15929, t15931)
}
