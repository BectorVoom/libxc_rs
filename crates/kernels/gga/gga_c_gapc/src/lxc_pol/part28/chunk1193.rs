//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1193/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1193<F: Float>(t34726: F, t34729: F, t34732: F, t34735: F, t34745: F, t34747: F, t34749: F, t34752: F, t34755: F, t34757: F, t34759: F, t34761: F, t34764: F, t34767: F, t34742: F, t34779: F) -> (F, F, F, F, F, F) {
    let t37051 = 0.69504740211613770836e-3 * t34726;
    let t37052 = 0.34752370105806885418e-3 * t34729;
    let t37053 = 0.67402122125348062552e-7 * t34732;
    let t37054 = 0.11372686522837130914e-5 * t34735;
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
    let t37068 = -0.98332751566569010434e-7 * t34742 - t37058 - t37059 - t37060 + t37061 + t37062 - t37063 + t37064 - t37065 - t37066 - t37067;
    let t37072 = 0.13913017666225690434e-3 * t34779;
    (t37051, t37052, t37053, t37054, t37068, t37072)
}
