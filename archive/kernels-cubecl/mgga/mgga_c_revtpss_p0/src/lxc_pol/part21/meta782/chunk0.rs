//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2800/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2800<F: Float>(t14524: F, t39575: F, t10867: F, t1568: F, t14939: F, t233: F, t689: F, t869: F, t10069: F, t14588: F, t10518: F, t14606: F) -> (F, F, F, F, F) {
    let t51483 = t39575 * t14524;
    let t51484 = F::cast_from(0.34697458558045176417e-2_f64) * t51483;
    let t51498 = t10867 * t1568;
    let t51505 = t689 * t869 * t233 * t14939;
    let t51507 = t10069 * t14588;
    let t51512 = t14606 * t10518;
    (t51484, t51498, t51505, t51507, t51512)
}
