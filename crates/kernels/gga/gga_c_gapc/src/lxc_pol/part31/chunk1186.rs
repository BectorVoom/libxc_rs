//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1186/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1186<F: Float>(t34745: F, t34747: F, t34749: F, t34752: F, t34755: F, t34757: F, t34759: F, t34761: F, t34764: F, t34767: F, t34779: F, t34782: F, t34785: F, t34788: F, t34791: F, t34794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37058 = 0.13506074236995523433e-5 * t34745;
    let t37059 = 0.1011909669415296852e-6 * t34747;
    let t37060 = 0.15458908518028544927e-5 * t34749;
    let t37061 = 0.80966145833333333338e-4 * t34752;
    let t37062 = 0.40483072916666666668e-3 * t34755;
    let t37063 = 0.4637672555408563478e-4 * t34757;
    let t37064 = 0.65654527540950461132e-8 * t34759;
    let t37065 = 0.69685742139248181696e-4 * t34761;
    let t37066 = 0.63350674672043801542e-5 * t34764;
    let t37067 = 0.63350674672043801542e-5 * t34767;
    let t37072 = 0.13913017666225690434e-3 * t34779;
    let t37073 = 0.20041830772435757309e-6 * t34782;
    let t37074 = 0.69504740211613770836e-3 * t34785;
    let t37075 = 0.50083268227528753081e-5 * t34788;
    let t37076 = 0.43440462632258606772e-4 * t34791;
    let t37077 = 0.11372686522837130914e-4 * t34794;
    (t37058, t37059, t37060, t37061, t37062, t37063, t37064, t37065, t37066, t37067, t37072, t37073, t37074, t37075, t37076, t37077)
}
