//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 776/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk776(t35885: f64, t7653: f64, t7641: f64, t35889: f64, t7648: f64, t7633: f64, t2103: f64, t35864: f64, t2115: f64, t35876: f64, t2118: f64, t35925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36063 = t7653 * t35885;
    let t36065 = t7641 * t35885;
    let t36072 = t7648 * t35889;
    let t36074 = t7633 * t35889;
    let t36078 = t2103 * t35864;
    let t36088 = t2115 * t35876;
    let t36090 = t2118 * t35925;
    (t36063, t36065, t36072, t36074, t36078, t36088, t36090)
}
