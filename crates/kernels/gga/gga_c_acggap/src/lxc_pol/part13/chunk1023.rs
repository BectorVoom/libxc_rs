//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1023/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1023<F: Float>(t1426: F, t1579: F, t2085: F, t598: F, t4959: F, t7647: F, t30148: F, t5606: F, t7585: F, t7842: F, t1181: F, t23745: F, t604: F, t7493: F) -> (F, F, F, F) {
    let t34089 = t598 * t1426 * t1579 * t2085;
    let t34091 = t7647 * t4959;
    let t34092 = F::new(0.17149607247227894789e-2) * t34091;
    let t34095 = t7585 * t7842 * t30148 * t5606;
    let t34099 = t7493 * t1181 * t604 * t23745;
    (t34089, t34092, t34095, t34099)
}
