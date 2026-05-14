//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 992/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk992<F: Float>(t13880: F, t2944: F, t2960: F, t4625: F, t934: F, t2952: F, t4700: F, t287: F, t330: F, t250: F, t3106: F, t4711: F, t659: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t9700: F, t9702: F, t9708: F, t9710: F, t9712: F) -> (F, F, F, F, F, F) {
    let t13881 = t13880 * t2944;
    let t13885 = t2960 * t4625;
    let t13886 = t13885 * t934;
    let t13888 = t4700 * t2952;
    let t13890 = t287 * t330;
    let t13892 = t250 * t3106 * t13890;
    let t13908 = t659 * t4711;
    let t13909 = 0.21908444444444444444e0 * t13908;
    let t13910 = -0.19931111111111111111e0 * t9700 - 0.10954222222222222222e0 * t9702 - 0.18257037037037037037e0 * t9708 + 0.54771111111111111111e-1 * t9710 + 0.18257037037037037037e-1 * t9712 - 0.19931111111111111111e0 * t13729 - 0.33218518518518518518e0 * t13720 - 0.79724444444444444445e0 * t13726 + 0.59793333333333333334e0 * t13738 + 0.23917333333333333334e1 * t13735 - t13909;
    (t13881, t13886, t13888, t13892, t13908, t13910)
}
