//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1176/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1176<F: Float>(t1036: F, t17744: F, t17422: F, t3020: F, t17500: F, t3061: F, t1085: F, t17360: F, t1066: F, t17777: F, t18190: F, t34029: F) -> (F, F, F, F, F, F) {
    let t52890 = t17744 * t1036;
    let t53039 = t17422 * t3020;
    let t53108 = t17500 * t3061;
    let t53152 = t17360 * t1085;
    let t53155 = t17777 * t1066;
    let t53260 = t34029 * t18190;
    (t52890, t53039, t53108, t53152, t53155, t53260)
}
