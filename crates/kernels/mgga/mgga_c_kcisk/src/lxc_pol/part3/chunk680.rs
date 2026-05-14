//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 680/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk680<F: Float>(t11412: F, t4658: F, t4629: F, t10459: F, t707: F, t10463: F, t708: F, t10441: F, t3521: F, t4616: F, t4652: F, t682: F, t1824: F, t4630: F, t4684: F, t10449: F, t1876: F, t1877: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11413 = t11412 * t4658;
    let t11414 = t4629 * t11413;
    let t11417 = t10459 * t707;
    let t11418 = t708 * t10463;
    let t11420 = t11417 * t11418 * t10441;
    let t11423 = t3521 * t4616;
    let t11425 = t682 * t4652;
    let t11426 = t11425 * t1824;
    let t11427 = t4629 * t11426;
    let t11430 = t4630 * t4684;
    let t11431 = t4629 * t11430;
    let t11435 = t1876 * t1877 * t10449;
    (t11413, t11414, t11420, t11423, t11426, t11427, t11430, t11431, t11435)
}
