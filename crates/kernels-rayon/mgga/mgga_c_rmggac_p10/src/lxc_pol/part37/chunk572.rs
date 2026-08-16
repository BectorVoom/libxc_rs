//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 572/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk572(t14953: f64, t289: f64, t14008: f64, t14016: f64, t14028: f64, t14036: f64, t14043: f64, t14049: f64, t14057: f64, t321: f64, t3282: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14954 = t289 * t14953;
    let t14955 = 0.2363e1_f64 * t14954;
    let t14956 = 0.52557918278704101561e-6_f64 * t14008;
    let t14957 = 0.87596530464506835932e-6_f64 * t14016;
    let t14958 = 0.87596530464506835932e-6_f64 * t14028;
    let t14959 = 0.17519306092901367187e-6_f64 * t14036;
    let t14960 = 0.43798265232253417968e-6_f64 * t14043;
    let t14961 = 0.87596530464506835932e-6_f64 * t14049;
    let t14962 = 0.87596530464506835932e-6_f64 * t14057;
    let t14969 = t3282 * t321;
    let t14970 = t739 * t14969;
    (t14955, t14956, t14957, t14958, t14959, t14960, t14961, t14962, t14969, t14970)
}
