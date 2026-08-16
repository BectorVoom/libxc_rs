//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 833/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk833(t7204: f64, t74960: f64, t15128: f64, t333: f64, t262: f64, t7192: f64, t15098: f64, t321: f64, t1326: f64, t68815: f64, t1322: f64, t235: f64, t26115: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74961 = t7204 * t74960;
    let t74963 = t15128 * t333;
    let t74964 = t262 * t74963;
    let t74965 = t7192 * t74964;
    let t74967 = t15098 * t321;
    let t74968 = t1326 * t74967;
    let t74969 = t68815 * t74968;
    let t74973 = t15098 * t333;
    let t74974 = t1326 * t74973;
    let t74975 = t235 * t26115 * t1322 * t74974;
    (t74961, t74963, t74964, t74965, t74967, t74968, t74969, t74973, t74974, t74975)
}
