//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 767/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk767(t36: f64, t4616: f64, t34805: f64, t648: f64, t305: f64, t35590: f64, t2115: f64, t35876: f64, t2118: f64, t35925: f64, t2100: f64, t2103: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35972 = t4616 * t36;
    let t36034 = t648 * t34805;
    let t36058 = t305 * t35590;
    let t36088 = t2115 * t35876;
    let t36090 = t2118 * t35925;
    let t36094 = t2100 * t35876;
    let t36096 = t2103 * t35925;
    (t35972, t36034, t36058, t36088, t36090, t36094, t36096)
}
