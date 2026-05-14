//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 982/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk982<F: Float>(t10690: F, t8590: F, t6880: F, t4761: F, t8607: F, t1980: F, t22564: F, t22567: F, t22570: F, t22573: F, t22578: F, t22581: F, t22586: F, t22589: F, t22594: F, t22695: F, t22698: F) -> (F, F, F) {
    let t22815 = t10690 * t8590;
    let t22816 = t22815 * t6880;
    let t22819 = t4761 * t8607;
    let t22820 = t22819 * t1980;
    let t22843 = -0.33218518518518518518e0 * t22567 + 0.11958666666666666667e1 * t22570 + 0.79724444444444444444e0 * t22573 - 0.17938e1 * t22578 - 0.23917333333333333334e1 * t22581 - 0.19931111111111111111e0 * t22586 + 0.59793333333333333334e0 * t22589 - 0.82156666666666666667e-1 * t22695 - 0.29896666666666666667e0 * t22594 + 0.18257037037037037037e-1 * t22698 + 0.66437037037037037037e-1 * t22564;
    (t22816, t22820, t22843)
}
