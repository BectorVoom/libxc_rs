//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 621/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk621(t678: f64, t7944: f64, t2153: f64, t275: f64, t1347: f64, t669: f64, t1288: f64, t668: f64, t72: f64, t2028: f64, t2604: f64, t7245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7945 = t7944 * t678;
    let t7946 = 0.42564599893297839398e-5_f64 * t7945;
    let t7947 = t275 * t2153;
    let t7949 = t1347 * t669;
    let t7950 = t1288 * t668;
    let t7951 = t72 * t7950;
    let t7952 = t2604 * t2028;
    let t7953 = 0.11974241701863808564e0_f64 * t7952;
    let t8026 = 0.39726959900411316772e-4_f64 * t7245;
    (t7946, t7947, t7949, t7950, t7951, t7953, t8026)
}
