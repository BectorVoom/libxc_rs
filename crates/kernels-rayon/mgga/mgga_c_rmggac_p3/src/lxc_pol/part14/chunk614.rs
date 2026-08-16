//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 614/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk614(t7906: f64, t2004: f64, t2186: f64, t2007: f64, t2191: f64, t1263: f64, t1986: f64, t675: f64, t2031: f64, t4041: f64, t1223: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7907 = 0.12769379967989351819e-4_f64 * t7906;
    let t7908 = t2186 * t2004;
    let t7910 = t2186 * t2007;
    let t7912 = t2191 * t2004;
    let t7913 = 0.85129199786595678796e-5_f64 * t7912;
    let t7914 = t1986 * t1263;
    let t7915 = t675 * t7914;
    let t7916 = 0.51077519871957407276e-4_f64 * t7915;
    let t7917 = t4041 * t2031;
    let t7918 = 0.11974241701863808564e0_f64 * t7917;
    let t7919 = t1223 * t28;
    (t7907, t7908, t7910, t7913, t7914, t7916, t7918, t7919)
}
