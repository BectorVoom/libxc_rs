//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1329/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1329<F: Float>(t11226: F, t654: F, t116320: F, t33056: F, t2788: F, t642: F, t34115: F, t5074: F, t34267: F, t9660: F, t1772: F, t4823: F, t7201: F, t15870: F, t34104: F, t32909: F, t34122: F) -> (F, F, F, F, F, F, F, F, F) {
    let t117090 = t11226 * t654;
    let t117106 = t33056 * t116320;
    let t117108 = t2788 * t642;
    let t117120 = t5074 * t34115;
    let t117121 = 0.14739506172839506172e-2 * t117120;
    let t117128 = 0.69444444444444444446e-2 * t34267 * t9660;
    let t117130 = t4823 * t7201 * t1772;
    let t117133 = t15870 * t34104;
    let t117136 = 0.69444444444444444446e-2 * t34122 * t32909;
    (t117090, t117106, t117108, t117120, t117121, t117128, t117130, t117133, t117136)
}
