//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 813/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk813<F: Float>(t13504: F, t13505: F, t12868: F, t6183: F, t4092: F, t45: F, t4120: F, t4126: F, t6125: F, t301: F, t342: F, t969: F) -> (F, F, F, F, F) {
    let t13506 = t13504 * t13505;
    let t13509 = t6183 * t12868;
    let t13512 = t45 * t4092;
    let t13518 = t4126 * t4120 * t6125;
    let t13522 = t342 * t969 * t301;
    (t13506, t13509, t13512, t13518, t13522)
}
