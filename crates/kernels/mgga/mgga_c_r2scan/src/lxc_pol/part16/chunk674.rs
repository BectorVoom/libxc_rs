//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 674/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk674<F: Float>(t1647: F, t1891: F, t5679: F, t4831: F, t4832: F, t4833: F, t4834: F, t4835: F, t5309: F, t5312: F, t5315: F, t650: F, t653: F, t685: F, t63: F, t688: F) -> (F, F, F, F, F) {
    let t5682 = 0.1551780387578202009e4 * t1891 * t5679 * t1647;
    let t5686 = -0.126595e1 * t5309 + 0.84396666666666666667e0 * t5312 - 0.3938511111111111111e1 * t5315 - t4831 + t4832 - t4833 - t4834 - t4835;
    let t5689 = 0.16081979498692535067e2 * t650 * t653 * t5686;
    let t5693 = t685 * t685;
    let t5694 = 1.0 / t5693;
    let t5695 = t63 * t5694;
    let t5696 = t688 * t688;
    (t5682, t5686, t5689, t5695, t5696)
}
