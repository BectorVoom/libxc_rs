//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1396/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1396<F: Float>(t34755: F, t34757: F, t34759: F, t34761: F, t34764: F, t34767: F, t34779: F, t34782: F, t34785: F, t34788: F, t34791: F, t34794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37062 = F::cast_from(0.40483072916666666668e-3_f64) * t34755;
    let t37063 = F::cast_from(0.4637672555408563478e-4_f64) * t34757;
    let t37064 = F::cast_from(0.65654527540950461132e-8_f64) * t34759;
    let t37065 = F::cast_from(0.69685742139248181696e-4_f64) * t34761;
    let t37066 = F::cast_from(0.63350674672043801542e-5_f64) * t34764;
    let t37067 = F::cast_from(0.63350674672043801542e-5_f64) * t34767;
    let t37072 = F::cast_from(0.13913017666225690434e-3_f64) * t34779;
    let t37073 = F::cast_from(0.20041830772435757309e-6_f64) * t34782;
    let t37074 = F::cast_from(0.69504740211613770836e-3_f64) * t34785;
    let t37075 = F::cast_from(0.50083268227528753081e-5_f64) * t34788;
    let t37076 = F::cast_from(0.43440462632258606772e-4_f64) * t34791;
    let t37077 = F::cast_from(0.11372686522837130914e-4_f64) * t34794;
    (t37062, t37063, t37064, t37065, t37066, t37067, t37072, t37073, t37074, t37075, t37076, t37077)
}
