//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 987/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk987<F: Float>(t311: F, t8950: F, t10: F, t3145: F, t2917: F, t8700: F, t106: F, t1141: F, t116: F, t3241: F, t3242: F, t11899: F, t2849: F) -> (F, F, F, F, F, F) {
    let t12042 = t311 * t8950;
    let t12068 = t10 * t3145;
    let t12478 = t8700 * t2917;
    let t12532 = t106 * t1141;
    let t12567 = t3241 * t3242 * t116;
    let t12568 = t11899 * t2849;
    (t12042, t12068, t12478, t12532, t12567, t12568)
}
