//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1150/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1150<F: Float>(t23882: F, t23897: F, t23918: F, t23940: F, t828: F, t837: F, t845: F, t549: F, t6541: F) -> (F, F, F) {
    let t23942 = t23882 + t23897 + t23918 + t23940;
    let t23946 = F::new(0.58482233974552040708e0) * t845 * t828 * t23942 * t837;
    let t23951 = t6541 * t549;
    (t23942, t23946, t23951)
}
