//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 874/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk874<F: Float>(t19104: F, t19483: F, t19102: F, t45: F, t5761: F, t1341: F, t3795: F, t1337: F, t140: F, t15868: F, t5603: F, t3748: F, t6011: F, t3480: F, t5598: F, t5613: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19632 = 0.23744444444444444444e-1 * t19104;
    let t19645 = 0.22076e0 * t19483;
    let t19678 = 0.13418888888888888889e0 * t19102;
    let t19706 = t45 * t5761;
    let t19720 = t3795 * t1341;
    let t19734 = t140 * t15868 * t1337;
    let t19735 = t19734 * t5603;
    let t19737 = t3748 * t6011;
    let t19738 = 0.22109259259259259258e-2 * t19737;
    let t19740 = t140 * t5598 * t3480;
    let t19757 = t3748 * t5613;
    (t19632, t19645, t19678, t19706, t19720, t19734, t19735, t19737, t19738, t19740, t19757)
}
