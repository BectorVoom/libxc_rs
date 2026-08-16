//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1271/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1271(t3765: f64, t6199: f64, t8205: f64, t851: f64, t10182: f64, t8170: f64, t898: f64, t3157: f64, t9762: f64, t1208: f64, t889: f64, t22180: f64, t27675: f64) -> (f64, f64, f64, f64, f64) {
    let t31052 = 0.1551780387578202009e4_f64 * t6199 * t3765 * t8205 * t851;
    let t31055 = 0.51947577317044391277e2_f64 * t898 * t10182 * t8170;
    let t31057 = 0.17544670867903938621e1_f64 * t9762 * t3157;
    let t31058 = t1208 * t889;
    let t31061 = 0.30762056574649219973e4_f64 * t22180 * t27675 * t31058;
    (t31052, t31055, t31057, t31058, t31061)
}
