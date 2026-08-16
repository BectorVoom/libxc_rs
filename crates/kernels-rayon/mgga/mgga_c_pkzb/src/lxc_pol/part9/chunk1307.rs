//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1307/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1307(t2401: f64, t3206: f64, t3208: f64, t8441: f64, t926: f64, t19191: f64, t2380: f64, t3195: f64, t2029: f64, t8309: f64, t3199: f64, t5728: f64) -> (f64, f64, f64, f64, f64) {
    let t22944 = t3206 * t2401 * t3208;
    let t22945 = 0.14291339372689912324e-3_f64 * t22944;
    let t22947 = t3206 * t926 * t8441;
    let t22950 = t2380 * t19191 * t3195;
    let t22951 = 0.28582678745379824648e-3_f64 * t22950;
    let t22952 = t8309 * t2029;
    let t22957 = t3199 * t5728;
    (t22945, t22947, t22951, t22952, t22957)
}
