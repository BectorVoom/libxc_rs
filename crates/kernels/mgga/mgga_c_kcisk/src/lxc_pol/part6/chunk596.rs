//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 596/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk596<F: Float>(t604: F, t1776: F, t7718: F, t1775: F, t5007: F, t7715: F, t5006: F, t2464: F, t5031: F, t1310: F, t8616: F, t1783: F, t2448: F, t2455: F, t652: F, t742: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t659 = 0.0 < t604;
    let t8806 = t1776 * t7718;
    let t8807 = t1775 * t8806;
    let t8810 = t5007 * t7715;
    let t8811 = t5006 * t8810;
    let t8814 = t2464 * t2464;
    let t8815 = t5031 * t8814;
    let t8816 = t1310 * t8815;
    let t8820 = piecewise3(t659, t8616, -t8616);
    let t8821 = t1783 * t8820;
    let t8822 = t1310 * t8821;
    let t8825 = t2448 * t2455;
    let t8831 = 1.0 / t652 / t742;
    (t8806, t8807, t8810, t8811, t8814, t8815, t8816, t8820, t8821, t8822, t8825, t8831)
}
