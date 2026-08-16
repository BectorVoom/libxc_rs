//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 806/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk806(t3056: f64, t3057: f64, t8858: f64, t8862: f64, t15238: f64, t5016: f64, t2044: f64, t558: f64, t7273: f64, t7554: f64, t2084: f64, t2145: f64, t2367: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t74514 = t3056 * t3057 * t8858;
    let t74517 = t3056 * t3057 * t8862;
    let t74520 = 0.5987120850931904282e-1_f64 * t5016 * t15238;
    let t74523 = t7273 * t2044 * t7554 * t558;
    let t74533 = t2145 * t27 * t2084 * t2367;
    (t74514, t74517, t74520, t74523, t74533)
}
