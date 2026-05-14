//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1028/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1028<F: Float>(t14199: F, t6225: F, t3482: F, t2231: F, t382: F, t3742: F, t14265: F, t19067: F, t3764: F, t5967: F, t1415: F, t1411: F, t1375: F, t19119: F, t5857: F, t970: F) -> (F, F, F, F, F, F, F, F) {
    let t20631 = t14199 * t6225;
    let t20632 = t3482 * t20631;
    let t20634 = t382 * t2231;
    let t20635 = t20634 * t3742;
    let t20636 = t14265 * t20635;
    let t20637 = t19067 * t20636;
    let t20639 = t3764 * t5967;
    let t20640 = t1415 * t20639;
    let t20641 = t1411 * t20640;
    let t20657 = t1375 * t19119;
    let t20660 = t970 * t5857;
    (t20632, t20634, t20635, t20637, t20639, t20641, t20657, t20660)
}
