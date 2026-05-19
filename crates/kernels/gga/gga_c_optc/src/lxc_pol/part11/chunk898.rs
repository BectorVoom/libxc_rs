//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 898/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk898<F: Float>(t16784: F, t7341: F, t2476: F, t845: F, t2472: F, t837: F, t13614: F, t3780: F, t7501: F, t7504: F, t3788: F, t4856: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16928 = t7341 * t16784;
    let t16929 = t16928 * t2476;
    let t16931 = F::cast_from(0.1038945353962551798e3_f64) * t845 * t16929;
    let t16933 = t2472 * t16784 * t837;
    let t16935 = F::cast_from(0.35089340384731224426e1_f64) * t845 * t16933;
    let t16939 = t13614 * t3780;
    let t16941 = F::cast_from(0.51947267698127589897e2_f64) * t845 * t16939;
    let t16942 = t7501 * t16784;
    let t16943 = t16942 * t7504;
    let t16945 = F::cast_from(0.1025389702100779493e4_f64) * t845 * t16943;
    let t16947 = F::cast_from(0.35089340384731224426e1_f64) * t3788 * t4856;
    (t16929, t16931, t16933, t16935, t16939, t16941, t16943, t16945, t16947)
}
