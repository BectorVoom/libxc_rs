//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1263/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1263(t2529: f64, t56689: f64, t837: f64, t845: f64, t17041: f64, t3788: f64, t2476: f64, t56677: f64, t7501: f64, t4920: f64, t4957: f64, t13998: f64, t4815: f64) -> (f64, f64, f64, f64, f64) {
    let t56939 = 0.35089340384731224426e1_f64 * t845 * t2529 * t56689 * t837;
    let t56941 = 0.14035736153892489771e2_f64 * t3788 * t17041;
    let t56945 = 0.6233672123775310788e3_f64 * t845 * t7501 * t56677 * t2476;
    let t56948 = 0.21053604230838734656e2_f64 * t845 * t4957 * t4920;
    let t56950 = 6.0_f64 * t13998 * t4815;
    (t56939, t56941, t56945, t56948, t56950)
}
