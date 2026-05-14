//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1199/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1199<F: Float>(t35135: F, t35137: F, t35141: F, t35143: F, t35146: F, t35152: F, t35155: F, t35157: F, t35160: F, t35162: F, t35149: F, t35169: F, t35173: F, t35184: F, t35186: F, t35188: F) -> (F, F, F, F, F, F) {
    let t37210 = 0.43284943850479925795e-3 * t35135;
    let t37211 = 0.3243554543208642639e-2 * t35137;
    let t37212 = 0.61551119569641057312e-8 * t35141;
    let t37213 = 0.27012148473991046866e-5 * t35143;
    let t37214 = 0.11372686522837130914e-5 * t35146;
    let t37216 = 0.337303223138432284e-8 * t35152;
    let t37217 = 0.55331893559454114829e-8 * t35155;
    let t37218 = 0.99044544404633838508e-5 * t35157;
    let t37219 = 0.33816362383187442026e-5 * t35160;
    let t37220 = 0.80192315782160920384e-6 * t35162;
    let t37221 = -t37210 - t37211 - t37212 - t37213 + t37214 - 0.64456181686737100543e-8 * t35149 + t37216 + t37217 + t37218 + t37219 - t37220;
    let t37223 = 0.11984097313886885523e-6 * t35169;
    let t37224 = 0.63350674672043801542e-5 * t35173;
    let t37227 = 0.69504740211613770836e-3 * t35184;
    let t37228 = 0.34752370105806885418e-3 * t35186;
    let t37229 = 0.34782544165564226085e-4 * t35188;
    (t37221, t37223, t37224, t37227, t37228, t37229)
}
