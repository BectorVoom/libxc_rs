//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 855/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk855<F: Float>(t309: F, t8306: F, t3919: F, t8347: F, t29991: F, t639: F, t7987: F, t8104: F, t2131: F, t2132: F, t3644: F, t633: F, t2138: F, t2217: F, t879: F, t847: F) -> (F, F, F, F, F, F, F) {
    let t33232 = t8306 * t309;
    let t33250 = t8347 * t3919;
    let t33256 = t29991 * t639;
    let t33266 = t7987 * t8104;
    let t33271 = 0.8673628188205199462e0 * t2131 * t2132 * t633 * t3644;
    let t33274 = t2138 * t2132 * t2217 * t879;
    let t33278 = t2131 * t2132 * t2217 * t847;
    (t33232, t33250, t33256, t33266, t33271, t33274, t33278)
}
