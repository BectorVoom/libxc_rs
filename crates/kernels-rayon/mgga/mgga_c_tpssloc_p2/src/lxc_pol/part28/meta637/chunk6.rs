//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2038/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2038(t2098: f64, t2319: f64, t111: f64, t7945: f64, t12524: f64, t12813: f64, t1458: f64, t16535: f64, t16538: f64, t16541: f64, t20173: f64, t2039: f64, t23917: f64, t24465: f64, t27170: f64, t27273: f64, t27276: f64, t27281: f64, t3938: f64, t3941: f64, t4072: f64, t45560: f64, t55341: f64, t55571: f64, t577: f64, t66940: f64, t7056: f64, t7230: f64, t7801: f64, t7956: f64, t94106: f64) -> f64 {
    let t94165 = t2098 * t2319;
    let t94170 = t7945 * t111;
    let t94202 = 27.0_f64 * t45560 * t7956 + 0.135e2_f64 * t55341 * t2039 + 27.0_f64 * t94165 * t1458 + 0.135e2_f64 * t7230 * t12813 + 27.0_f64 * t94170 * t2319 + 54.0_f64 * t66940 * t7956 + 27.0_f64 * t3941 * t23917 * t1458 + 54.0_f64 * t3941 * t7056 * t4072 + 0.45e1_f64 * t94106 * t577 + 54.0_f64 * t24465 * t16538 + 27.0_f64 * t24465 * t16541 + 54.0_f64 * t12524 * t27281 + 27.0_f64 * t3941 * t2039 * t12813 + 27.0_f64 * t55571 * t7956 + 54.0_f64 * t20173 * t27273 + 54.0_f64 * t20173 * t27276 + 27.0_f64 * t16535 * t7801 + 27.0_f64 * t3938 * t27170;
    t94202
}
