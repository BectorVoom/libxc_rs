//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1070/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1070<F: Float>(t46978: F, t8033: F, t2173: F, t27867: F, t2822: F, t27870: F, t15573: F, t27914: F, t8041: F, t7690: F, t14654: F, t3489: F, t27785: F, t27864: F, t3245: F, t8057: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t96305 = t46978 * t8033;
    let t96306 = t2173 * t96305;
    let t96339 = t2822 * t27867;
    let t96340 = 0.14739506172839506172e-2 * t96339;
    let t96345 = t2822 * t27870;
    let t96356 = t15573 * t27914;
    let t96358 = 0.46336805555555555556e-3 * t2173 * t96356;
    let t96382 = t2173 * t46978 * t8041;
    let t96388 = t7690 * t96305;
    let t96391 = t14654 * t3489;
    let t96395 = t2822 * t27785;
    let t96396 = 0.14739506172839506172e-2 * t96395;
    let t96401 = t2822 * t27864;
    let t96402 = 0.22109259259259259258e-2 * t96401;
    let t96412 = t3245 * t8057;
    (t96306, t96339, t96340, t96345, t96356, t96358, t96382, t96388, t96391, t96395, t96396, t96401, t96402, t96412)
}
