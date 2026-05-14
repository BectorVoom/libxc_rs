//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1261/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1261<F: Float>(t10500: F, t2790: F, t18681: F, t2782: F, t2784: F, t32955: F, t32990: F, t32958: F, t4998: F, t9664: F, t3805: F, t9684: F, t1871: F, t654: F, t642: F, t1772: F, t46460: F, t648: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112551 = t10500 * t2790;
    let t112552 = 0.73697530864197530862e-3 * t112551;
    let t112560 = 0.38580246913580246915e-2 * t2782 * t18681 * t2784;
    let t112571 = t32990 * t32955;
    let t112574 = t9664 * t4998 * t32958;
    let t112576 = t3805 * t9684;
    let t112585 = t1871 * t654;
    let t112586 = t112585 * t642;
    let t112591 = t46460 * t648 * t1772;
    (t112551, t112552, t112560, t112571, t112574, t112576, t112585, t112586, t112591)
}
