//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 774/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk774(t2118: f64, t35925: f64, t2115: f64, t35872: f64, t2100: f64, t35876: f64, t2103: f64, t35864: f64, t25518: f64, t27: f64, t25640: f64, t25636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36090 = t2118 * t35925;
    let t36092 = t2115 * t35872;
    let t36094 = t2100 * t35876;
    let t36096 = t2103 * t35925;
    let t36099 = t2118 * t35864;
    let t36101 = t2100 * t35872;
    let t36103 = t25518 * t27;
    let t36107 = t25640 * t27;
    let t36110 = t25636 * t27;
    (t36090, t36092, t36094, t36096, t36099, t36101, t36103, t36107, t36110)
}
