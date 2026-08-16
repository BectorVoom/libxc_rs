//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 557/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk557<F: Float>(t1788: F, t3393: F, t148: F, t41: F, t85: F, t2840: F, t339: F, t4567: F, t1154: F, t1646: F, t3405: F, t1018: F) -> (F, F, F, F, F, F) {
    let t5130 = t3393 * t1788;
    let t5133 = t85 * t148 * t41;
    let t5134 = t2840 * t339;
    let t5135 = t5134 * t4567;
    let t5139 = t1154 * t3405 * t1646;
    let t5142 = t1018 * t339;
    (t5130, t5133, t5134, t5135, t5139, t5142)
}
