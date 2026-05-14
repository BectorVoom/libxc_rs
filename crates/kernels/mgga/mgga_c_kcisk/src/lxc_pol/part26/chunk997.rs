//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 997/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk997<F: Float>(t26813: F, t5675: F, t19720: F, t3482: F, t5670: F, t5633: F, t2152: F, t220: F, t3485: F, t3484: F, t19740: F, t19951: F, t2232: F, t1411: F, t3512: F, t7832: F) -> (F, F, F, F, F, F, F, F) {
    let t26814 = t5675 * t26813;
    let t26815 = t19720 * t26814;
    let t26816 = t3482 * t26815;
    let t26818 = t5670 * t26813;
    let t26819 = t19720 * t26818;
    let t26820 = t5633 * t26819;
    let t26822 = t220 * t2152;
    let t26823 = t3485 * t26822;
    let t26824 = t3484 * t26823;
    let t26825 = t19740 * t26824;
    let t26827 = t19951 * t2232;
    let t26828 = t1411 * t26827;
    let t26830 = t3512 * t7832;
    (t26814, t26816, t26818, t26820, t26823, t26825, t26828, t26830)
}
