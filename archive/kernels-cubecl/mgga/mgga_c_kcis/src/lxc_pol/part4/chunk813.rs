//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 813/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk813<F: Float>(t2840: F, t339: F, t4567: F, t1154: F, t1646: F, t3405: F, t1018: F, t4581: F, t1155: F, t167: F, t1791: F, t238: F, t86: F) -> (F, F, F, F, F, F, F) {
    let t5134 = t2840 * t339;
    let t5135 = t5134 * t4567;
    let t5139 = t1154 * t3405 * t1646;
    let t5142 = t1018 * t339;
    let t5143 = t5142 * t4581;
    let t5147 = t1154 * t1155 * t167;
    let t5151 = t86 * t238 * t1791;
    (t5134, t5135, t5139, t5142, t5143, t5147, t5151)
}
