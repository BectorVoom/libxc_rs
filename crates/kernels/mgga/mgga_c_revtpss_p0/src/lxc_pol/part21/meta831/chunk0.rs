//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3100/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3100<F: Float>(t3623: F, t53739: F, t13127: F, t12865: F, t3746: F, t13396: F, t5405: F, t13392: F, t17672: F, t4181: F, t17677: F, t17682: F) -> (F, F, F, F, F, F, F, F) {
    let t56878 = t3623 * t53739;
    let t56879 = t13127 * t56878;
    let t56888 = t3746 * t12865;
    let t56891 = t13396 * t5405;
    let t56895 = t13392 * t5405;
    let t56899 = t4181 * t17672;
    let t56903 = t4181 * t17677;
    let t56907 = t4181 * t17682;
    (t56878, t56879, t56888, t56891, t56895, t56899, t56903, t56907)
}
