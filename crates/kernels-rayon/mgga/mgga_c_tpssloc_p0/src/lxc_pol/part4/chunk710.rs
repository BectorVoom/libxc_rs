//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 710/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk710(t460: f64, t4935: f64, t4934: f64, t1174: f64, t1180: f64, t1187: f64, t3430: f64, t3433: f64, t3436: f64, t3447: f64, t4887: f64, t4889: f64, t4897: f64, t4901: f64, t4905: f64, t4909: f64, t4913: f64, t4917: f64, t4920: f64, t4931: f64) -> (f64, f64, f64) {
    let t4936 = t4935 * t460;
    let t4937 = t4934 * t4936;
    let t4940 = -0.74074074074074074073e-3_f64 * t4887 + 0.74074074074074074073e-3_f64 * t4889 * t1180 + 0.22222222222222222222e-2_f64 * t4889 * t1187 - t3430 - 0.9259259259259259259e-4_f64 * t3433 - 0.27777777777777777777e-3_f64 * t3436 - 0.9259259259259259259e-4_f64 * t4897 + 0.37037037037037037036e-3_f64 * t3447 * t4901 + 0.27777777777777777777e-3_f64 * t3447 * t4905 - 0.55555555555555555554e-3_f64 * t3447 * t4909 - 0.27777777777777777777e-3_f64 * t1174 * t4913 - 0.27777777777777777777e-3_f64 * t4917 + 0.27777777777777777777e-3_f64 * t3447 * t4920 - 0.83333333333333333332e-3_f64 * t1174 * t4931 - 0.83333333333333333332e-3_f64 * t1174 * t4937;
    (t4936, t4937, t4940)
}
