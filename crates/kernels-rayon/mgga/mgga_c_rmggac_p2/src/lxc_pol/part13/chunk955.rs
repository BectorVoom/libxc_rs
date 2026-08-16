//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 955/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk955(t558: f64, t7817: f64, t797: f64, t305: f64, t38381: f64, t39879: f64, t5271: f64, t262: f64, t40802: f64, t7835: f64, t35815: f64, t39662: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40963 = t5271 * t39879;
    let t40965 = t262 * t40802;
    let t40966 = t7835 * t40965;
    let t40968 = t35815 * t39662;
    (t40948, t40949, t40951, t40963, t40965, t40966, t40968)
}
