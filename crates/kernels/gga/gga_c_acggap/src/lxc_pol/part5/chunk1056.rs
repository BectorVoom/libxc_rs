//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1056/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1056<F: Float>(t12752: F, t1562: F, t3382: F, t4439: F, t1347: F, t3237: F, t4932: F, t997: F, t1418: F, t5260: F, t1165: F, t3491: F, t4282: F, t540: F) -> (F, F, F, F, F, F, F) {
    let t18555 = t12752 * t1562;
    let t18566 = t3382 * t4439;
    let t18578 = t3237 * t1347;
    let t18580 = t997 * t4932;
    let t18582 = t3237 * t1418;
    let t18584 = t997 * t5260;
    let t18588 = t4282 * t1165 * t540 * t3491;
    (t18555, t18566, t18578, t18580, t18582, t18584, t18588)
}
