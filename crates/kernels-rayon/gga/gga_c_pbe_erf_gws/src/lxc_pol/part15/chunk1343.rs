//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1343/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1343(t4135: f64, t51966: f64, t22410: f64, t2409: f64, t3959: f64, t22192: f64, t3965: f64, t9220: f64, t26885: f64, t2242: f64, t4185: f64, t1146: f64, t13987: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54621 = t51966 * t4135;
    let t54624 = t3959 * t2409 * t22410;
    let t54627 = t3965 * t2409 * t22192;
    let t54629 = t3959 * t9220;
    let t54636 = t3965 * t2409 * t26885;
    let t54639 = t2242 * t4185;
    let t54641 = t13987 * t1146;
    (t54621, t54624, t54627, t54629, t54636, t54639, t54641)
}
