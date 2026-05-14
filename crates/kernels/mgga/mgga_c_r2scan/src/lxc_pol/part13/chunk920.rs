//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 920/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk920<F: Float>(t10710: F, t8128: F, t10768: F, t10781: F, t2547: F, t2207: F, t3336: F, t3606: F, t1060: F, t2526: F, t1058: F, t3333: F, t7601: F, t10846: F, t10850: F, t10854: F, t10857: F, t10864: F, t10867: F) -> (F, F, F) {
    let t11816 = t10710 * t8128;
    let t11817 = t10768 * t11816;
    let t11819 = t10781 * t2547;
    let t11822 = t2207 * t3336 * t3606;
    let t11824 = t1060 * t2526;
    let t11826 = t2207 * t1058 * t11824;
    let t11831 = t7601 * t3333;
    let t11833 = 0.23804984598836975486e-2 * t11817 + 0.54878743191129263322e-1 * t11819 + 0.65495539973149862688e-2 * t11822 + 0.65495539973149862688e-2 * t11826 - 0.23287303101564395623e-1 * t10846 - 0.69861909304693186869e-1 * t10850 - t10854 - 0.48787202696913915093e-2 * t10857 + 0.21831846657716620896e-2 * t11831 + t10864 + t10867;
    (t11816, t11824, t11833)
}
