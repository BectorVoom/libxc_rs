//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1052/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1052<F: Float>(t24890: F, t5409: F, t24898: F, t5330: F, t15369: F, t1476: F, t5309: F, t2843: F, t840: F, t10683: F, t31572: F, t319: F, t5393: F, t6353: F, t296: F, t31643: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31804 = t24890 * t5409;
    let t31807 = t24898 * t5330;
    let t31808 = t15369 * t31807;
    let t31814 = t1476 * t5309;
    let t31816 = t840 * t2843 * t31814;
    let t31820 = t10683 * t319 * t31572;
    let t31824 = t6353 * t5393;
    let t31825 = t296 * t31824;
    let t31828 = t296 * t31643;
    (t31804, t31807, t31808, t31814, t31816, t31820, t31824, t31825, t31828)
}
