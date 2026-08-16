//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 737/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk737<F: Float>(t10463: F, t708: F, t10441: F, t11417: F, t3521: F, t4616: F, t4652: F, t682: F, t1824: F, t4629: F, t4630: F, t4684: F) -> (F, F, F, F, F) {
    let t11418 = t708 * t10463;
    let t11420 = t11417 * t11418 * t10441;
    let t11423 = t3521 * t4616;
    let t11425 = t682 * t4652;
    let t11426 = t11425 * t1824;
    let t11427 = t4629 * t11426;
    let t11430 = t4630 * t4684;
    (t11420, t11423, t11426, t11427, t11430)
}
