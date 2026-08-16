//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 934/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk934(t76912: f64, t2227: f64, t3351: f64, t515: f64, t618: f64, t7231: f64, t1528: f64, t698: f64, t14668: f64, t17859: f64, t14672: f64, t74219: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76913 = 0.53205749866622299248e-5_f64 * t76912;
    let t76917 = t3351 * t7231 * t515 * t2227 * t618;
    let t76918 = 0.42564599893297839398e-5_f64 * t76917;
    let t76922 = t3351 * t7231 * t515 * t698 * t1528;
    let t76923 = 0.42564599893297839398e-5_f64 * t76922;
    let t76924 = t17859 * t14668;
    let t76925 = 0.42564599893297839398e-5_f64 * t76924;
    let t76926 = t17859 * t14672;
    let t76927 = 0.12769379967989351819e-4_f64 * t76926;
    let t76928 = 0.1921128438866447784e-2_f64 * t74219;
    (t76913, t76918, t76923, t76925, t76927, t76928)
}
