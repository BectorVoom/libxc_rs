//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1397/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1397(t34755: f64, t34757: f64, t34759: f64, t34761: f64, t34764: f64, t34767: f64, t34779: f64, t34782: f64, t34785: f64, t34788: f64, t34791: f64, t34794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37062 = 0.40483072916666666668e-3_f64 * t34755;
    let t37063 = 0.4637672555408563478e-4_f64 * t34757;
    let t37064 = 0.65654527540950461132e-8_f64 * t34759;
    let t37065 = 0.69685742139248181696e-4_f64 * t34761;
    let t37066 = 0.63350674672043801542e-5_f64 * t34764;
    let t37067 = 0.63350674672043801542e-5_f64 * t34767;
    let t37072 = 0.13913017666225690434e-3_f64 * t34779;
    let t37073 = 0.20041830772435757309e-6_f64 * t34782;
    let t37074 = 0.69504740211613770836e-3_f64 * t34785;
    let t37075 = 0.50083268227528753081e-5_f64 * t34788;
    let t37076 = 0.43440462632258606772e-4_f64 * t34791;
    let t37077 = 0.11372686522837130914e-4_f64 * t34794;
    (t37062, t37063, t37064, t37065, t37066, t37067, t37072, t37073, t37074, t37075, t37076, t37077)
}
