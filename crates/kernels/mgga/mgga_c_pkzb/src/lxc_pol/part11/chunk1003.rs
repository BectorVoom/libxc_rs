//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1003/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1003<F: Float>(t4868: F, t7046: F, t1545: F, t2605: F, t1548: F, t16502: F, t16508: F, t2609: F, t5089: F, t135: F, t568: F, t5146: F, t1542: F, t16613: F, t16619: F, t16621: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19627 = t7046 * t4868;
    let t19687 = t1545 * t2605;
    let t19688 = 36.0 * t19687;
    let t19690 = 96.0 * t1548 * t2605;
    let t19695 = 12.0 * t16502;
    let t19697 = 144.0 * t16508;
    let t19702 = t2609 * t5089;
    let t19704 = t135 * t568;
    let t19710 = t2609 * t5146;
    let t19742 = t1542 * t2605;
    let t19743 = 60.0 * t19742;
    let t19748 = 240.0 * t16613;
    let t19751 = 36.0 * t16619;
    let t19752 = 96.0 * t16621;
    (t19627, t19688, t19690, t19695, t19697, t19702, t19704, t19710, t19743, t19748, t19751, t19752)
}
