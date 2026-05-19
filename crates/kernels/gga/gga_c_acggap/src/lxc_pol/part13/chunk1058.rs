//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1058/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1058<F: Float>(t5129: F, t7647: F, t5133: F, t2001: F, t4518: F, t4667: F, t5267: F, t5096: F, t5101: F, t7741: F, t1434: F, t7746: F) -> (F, F, F, F, F, F, F, F) {
    let t34534 = t7647 * t5129;
    let t34535 = F::cast_from(0.17149607247227894789e-2_f64) * t34534;
    let t34537 = t7647 * t5133;
    let t34538 = F::cast_from(0.85748036236139473944e-3_f64) * t34537;
    let t34539 = t2001 * t4518;
    let t34541 = t2001 * t4667;
    let t34543 = t2001 * t5267;
    let t34545 = t2001 * t5096;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    (t34535, t34538, t34539, t34541, t34543, t34545, t34547, t34549)
}
