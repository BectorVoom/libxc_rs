//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1407/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1407(t34755: f64, t34757: f64, t34759: f64, t34761: f64, t34764: f64, t34767: f64, t34742: f64, t37058: f64, t37059: f64, t37060: f64, t37061: f64, t34779: f64) -> (f64, f64) {
    let t37062 = 0.40483072916666666668e-3_f64 * t34755;
    let t37063 = 0.4637672555408563478e-4_f64 * t34757;
    let t37064 = 0.65654527540950461132e-8_f64 * t34759;
    let t37065 = 0.69685742139248181696e-4_f64 * t34761;
    let t37066 = 0.63350674672043801542e-5_f64 * t34764;
    let t37067 = 0.63350674672043801542e-5_f64 * t34767;
    let t37068 = -0.98332751566569010434e-7_f64 * t34742 - t37058 - t37059 - t37060 + t37061 + t37062 - t37063 + t37064 - t37065 - t37066 - t37067;
    let t37072 = 0.13913017666225690434e-3_f64 * t34779;
    (t37068, t37072)
}
