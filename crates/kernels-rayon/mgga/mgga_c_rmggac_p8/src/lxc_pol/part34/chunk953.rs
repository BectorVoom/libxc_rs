//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 953/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk953(t74203: f64, t15502: f64, t495: f64, t515: f64, t7230: f64, t7231: f64, t2227: f64, t3351: f64, t618: f64, t1528: f64, t698: f64, t14668: f64, t17859: f64) -> (f64, f64, f64, f64, f64) {
    let t76904 = 0.23268647941669485538e-4_f64 * t74203;
    let t76912 = t7230 * t7231 * t515 * t15502 * t495;
    let t76913 = 0.53205749866622299248e-5_f64 * t76912;
    let t76917 = t3351 * t7231 * t515 * t2227 * t618;
    let t76918 = 0.42564599893297839398e-5_f64 * t76917;
    let t76922 = t3351 * t7231 * t515 * t698 * t1528;
    let t76923 = 0.42564599893297839398e-5_f64 * t76922;
    let t76924 = t17859 * t14668;
    (t76904, t76913, t76918, t76923, t76924)
}
