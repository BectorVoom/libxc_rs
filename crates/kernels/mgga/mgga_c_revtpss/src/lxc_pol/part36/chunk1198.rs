//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1198/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1198<F: Float>(t22633: F, t94: F, t1937: F, t29508: F, t7735: F, t1907: F, t6816: F, t25082: F, t8717: F, t6941: F, t7953: F, t572: F, t5883: F, t7741: F, t7330: F, t105823: F, t5920: F) -> (F, F, F, F, F, F, F) {
    let t114812 = t94 * t22633;
    let t114814 = 2.0 * t114812 * t1937;
    let t114816 = 6.0 * t29508 * t7735;
    let t114820 = t6816 * t1907;
    let t114823 = 9.0 * t25082 * t8717 * t114820;
    let t114838 = 9.0 * t6941 * t7953;
    let t114841 = 18.0 * t572 * t5883 * t7741;
    let t114844 = 6.0 * t572 * t7330 * t22633;
    let t114847 = 18.0 * t572 * t105823 * t5920;
    (t114814, t114816, t114823, t114838, t114841, t114844, t114847)
}
