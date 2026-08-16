//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1372/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1372(t96261: f64, t96270: f64, t1262: f64, t26996: f64, t5329: f64, t5341: f64, t13173: f64, t15534: f64, t26955: f64, t26960: f64, t27010: f64, t28116: f64, t28137: f64, t28190: f64, t7772: f64, t7788: f64, t92761: f64, t93143: f64, t93145: f64, t96256: f64, t96259: f64, t96720: f64, t97141: f64) -> (f64, f64) {
    let t97352 = 0.61905925925925925925e-2_f64 * t96261;
    let t97360 = 0.61905925925925925925e-2_f64 * t96270;
    let t97366 = t5329 * t26996 * t5341 * t1262;
    let t97371 = 0.77382407407407407407e-3_f64 * t96256 - 0.185671721767578125e-4_f64 * t92761 * t28137 - 0.34822083333333333332e-2_f64 * t96259 + t97352 + 0.92673611111111111112e-3_f64 * t26960 * t15534 * t28116 * t13173 + 0.61836467013888888889e-4_f64 * t26955 * t97141 + 0.11349419753086419753e-1_f64 * t93143 - t97360 - 0.77382407407407407406e-3_f64 * t93145 + 0.208515625e-2_f64 * t7788 * t96720 - 0.92754700520833333334e-4_f64 * t7772 * t97366 - 0.11584201388888888889e-3_f64 * t28190 * t27010;
    (t97366, t97371)
}
