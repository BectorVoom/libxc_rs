//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1201/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1201<F: Float>(t17562: F, t17602: F, t17648: F, t17688: F, t552: F, t573: F, t12542: F, t2055: F, t17453: F, t5909: F, t4260: F, t2043: F, t4245: F, t16609: F, t584: F, t583: F, sigma2: F) -> (F, F, F, F, F) {
    let t17690 = t17562 + t17602 + t17648 + t17688;
    let t17691 = t17690 * t552;
    let t17692 = t17691 * sigma2;
    let t17693 = t17692 * t573;
    let t17695 = t12542 * t2055;
    let t17697 = t5909 * t17453;
    let t17698 = t4260 * t17697;
    let t17700 = t4245 * t2043;
    let t17702 = t584 * t16609;
    let t17703 = t583 * t17702;
    (t17693, t17695, t17698, t17700, t17703)
}
