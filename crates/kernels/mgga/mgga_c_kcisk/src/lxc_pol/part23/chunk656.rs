//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 656/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk656<F: Float>(t1173: F, t2188: F, t2083: F, t3598: F, t1175: F, t5684: F, t3573: F, t3611: F, t5668: F, t5673: F, t5678: F, t5682: F, t1355: F, t306: F, t3599: F, t3602: F, t5662: F) -> (F, F, F, F, F, F) {
    let t5687 = t1173 * t2188;
    let t5690 = t3598 * t2083;
    let t5691 = t5690 * t1175;
    let t5693 = t1173 * t5684;
    let t5700 = -0.991e-2 * t5691 + 0.1982e-1 * t5693 + t3611 + 0.13758333333333333333e-2 * t3573 + 0.13758333333333333333e-2 * t5668 - 0.27516666666666666667e-2 * t5673 + 0.8255e-2 * t5678 - 0.8255e-2 * t5682;
    let t5703 = -t3599 * t5662 / 8.0 + t3602 * t2083 / 4.0 + t1355 * t5684 / 4.0 + t5687 * t1175 / 4.0 + t306 * t5700 / 2.0;
    (t5687, t5690, t5691, t5693, t5700, t5703)
}
