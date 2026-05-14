//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 973/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk973<F: Float>(t19730: F, t5600: F, t1337: F, t140: F, t15868: F, t5603: F, t3748: F, t6011: F, t3480: F, t5598: F, t1286: F, t220: F, t3485: F, t3484: F, t19087: F, t5625: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19731 = t5600 * t19730;
    let t19734 = t140 * t15868 * t1337;
    let t19735 = t19734 * t5603;
    let t19737 = t3748 * t6011;
    let t19738 = 0.22109259259259259258e-2 * t19737;
    let t19740 = t140 * t5598 * t3480;
    let t19741 = t220 * t1286;
    let t19742 = t3485 * t19741;
    let t19743 = t3484 * t19742;
    let t19744 = t19740 * t19743;
    let t19746 = t5625 * t19087;
    (t19731, t19734, t19735, t19737, t19738, t19740, t19742, t19744, t19746)
}
