//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 932/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk932<F: Float>(t1676: F, t6847: F, t2386: F, t4787: F, t4790: F, t6874: F, t15993: F, t4761: F, t2597: F, t5372: F, t397: F, t963: F, t1774: F, t786: F, t2634: F, t5483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18553 = t6847 * t1676;
    let t18558 = t2386 * t4787;
    let t18565 = t6874 * t4790;
    let t18616 = 0.2283111111111111111e-1 * t15993;
    let t18640 = t2386 * t4761;
    let t18643 = t2597 * t5372;
    let t18681 = t397 * t963;
    let t18682 = t1774 * t786;
    let t18693 = 0.47975436576472845902e-1 * t2634 * t5483;
    (t18553, t18558, t18565, t18616, t18640, t18643, t18681, t18682, t18693)
}
