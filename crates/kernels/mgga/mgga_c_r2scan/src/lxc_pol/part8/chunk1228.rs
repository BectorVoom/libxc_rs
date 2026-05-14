//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1228/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1228<F: Float>(t2483: F, t625: F, t1764: F, t1768: F, t1416: F, t2794: F, t22428: F, t2743: F, t1726: F, t5364: F, t955: F, t22524: F, t22527: F, t897: F, t5714: F, t898: F) -> (F, F, F, F, F, F, F) {
    let t26448 = t2483 * t625;
    let t26449 = t26448 * t1764;
    let t26450 = 0.65061487801810439052e-1 * t26449;
    let t26451 = t26448 * t1768;
    let t26452 = 0.96319466275353142157e0 * t26451;
    let t26463 = t1416 * t2794;
    let t26475 = t2743 * t22428;
    let t26476 = 0.4051561992e0 * t26475;
    let t26481 = t1726 * t955 * t5364;
    let t26488 = t22524 * t897 * t22527;
    let t26490 = t898 * t5714;
    (t26450, t26452, t26463, t26476, t26481, t26488, t26490)
}
