//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 976/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk976<F: Float>(t34534: F, t5133: F, t7647: F, t5101: F, t7741: F, t1434: F, t7746: F, t4680: F, t7426: F, t8476: F, t30937: F, t8450: F) -> (F, F, F, F, F, F) {
    let t34535 = F::cast_from(0.17149607247227894789e-2_f64) * t34534;
    let t34537 = t7647 * t5133;
    let t34538 = F::cast_from(0.85748036236139473944e-3_f64) * t34537;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    let t34556 = t7426 * t4680 * t8476;
    let t34557 = F::cast_from(0.62896184579208304136e-3_f64) * t34556;
    let t34561 = t30937 * t8450;
    (t34535, t34538, t34547, t34549, t34557, t34561)
}
