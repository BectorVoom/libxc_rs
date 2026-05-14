//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 900/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk900<F: Float>(t1103: F, t2461: F, t1053: F, t1102: F, t10935: F, t3446: F, t970: F, t58: F, t897: F, t597: F, t10649: F, t10648: F, t10681: F, t10683: F, t10680: F, t10674: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11580 = t3446 * t10935 * t970;
    let t11582 = t58 * t897;
    let t11583 = t11582 * t597;
    let t11584 = t10649 * t11583;
    let t11585 = t10648 * t11584;
    let t11587 = t10681 * t897;
    let t11588 = t11587 * t10683;
    let t11589 = t10680 * t11588;
    let t11591 = t10674 * t897;
    (t11572, t11574, t11580, t11582, t11583, t11584, t11585, t11587, t11588, t11589, t11591)
}
