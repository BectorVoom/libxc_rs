//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1263/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1263<F: Float>(t2529: F, t56689: F, t837: F, t845: F, t17041: F, t3788: F, t2476: F, t56677: F, t7501: F, t4920: F, t4957: F, t13998: F, t4815: F) -> (F, F, F, F, F) {
    let t56939 = F::cast_from(0.35089340384731224426e1_f64) * t845 * t2529 * t56689 * t837;
    let t56941 = F::cast_from(0.14035736153892489771e2_f64) * t3788 * t17041;
    let t56945 = F::cast_from(0.6233672123775310788e3_f64) * t845 * t7501 * t56677 * t2476;
    let t56948 = F::cast_from(0.21053604230838734656e2_f64) * t845 * t4957 * t4920;
    let t56950 = F::new(6.0) * t13998 * t4815;
    (t56939, t56941, t56945, t56948, t56950)
}
