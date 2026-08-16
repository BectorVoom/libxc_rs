//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3194/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3194<F: Float>(t15572: F, t15740: F, t11697: F, t18382: F, t3577: F, t3575: F, t62053: F, t3624: F, t1229: F, t1734: F, t375: F, t3610: F) -> (F, F, F, F, F) {
    let t66360 = t15740 * t15572;
    let t66363 = t3577 * t11697 * t18382;
    let t66371 = t3575 * t62053;
    let t66372 = t3624 * t66371;
    let t66374 = t375 * t1229 * t1734;
    let t66378 = t3610 * t66371;
    (t66360, t66363, t66372, t66374, t66378)
}
