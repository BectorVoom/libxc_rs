//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1374/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1374(t27006: f64, t28190: f64, t26993: f64, t7788: f64, t93163: f64, t96318: f64, t96321: f64, t96324: f64, t96327: f64, t96330: f64, t96333: f64, t97056: f64, t97253: f64, t97366: f64) -> f64 {
    let t97407 = 0.7722800925925925926e-4_f64 * t28190 * t27006;
    let t97411 = 0.19345601851851851852e-2_f64 * t96318 + 0.51588271604938271605e-2_f64 * t96321 + 0.77382407407407407407e-2_f64 * t96324 - 0.23214722222222222222e-2_f64 * t96327 - 0.23214722222222222222e-2_f64 * t96330 - 0.69505208333333333334e-3_f64 * t7788 * t97366 - 0.23214722222222222222e-2_f64 * t96333 - 0.69505208333333333334e-3_f64 * t7788 * t97056 + 0.23168402777777777778e-3_f64 * t28190 * t26993 - t97407 - 0.34752604166666666667e-3_f64 * t7788 * t97253 + 0.20635308641975308642e-2_f64 * t93163;
    t97411
}
