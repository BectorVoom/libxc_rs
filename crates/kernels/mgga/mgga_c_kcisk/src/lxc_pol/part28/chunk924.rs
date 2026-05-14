//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 924/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk924<F: Float>(t17739: F, t10409: F, t6987: F, t6669: F, t140: F, t3737: F, t6672: F, t6677: F, t1907: F, t7291: F, t2541: F, t5217: F) -> (F, F, F, F, F, F, F, F) {
    let t17740 = 0.14739506172839506172e-2 * t17739;
    let t17750 = t10409 * t6987;
    let t17751 = 0.22109259259259259258e-2 * t17750;
    let t17757 = t10409 * t6669;
    let t17764 = t140 * t3737 * t6672;
    let t17765 = t17764 * t6677;
    let t17766 = 0.3684876543209876543e-2 * t17765;
    let t17772 = t7291 * t1907;
    let t17775 = t2541 * t5217;
    (t17740, t17750, t17751, t17757, t17765, t17766, t17772, t17775)
}
