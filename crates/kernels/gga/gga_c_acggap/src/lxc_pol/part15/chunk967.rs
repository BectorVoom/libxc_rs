//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 967/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk967<F: Float>(t1841: F, t7685: F, t1426: F, t429: F, t598: F, t9536: F, t137: F, t5506: F, t368: F, t1980: F, t38889: F, t7476: F, t1089: F, t2090: F, t22705: F, t34406: F, t5928: F) -> (F, F, F, F, F, F, F) {
    let t38916 = t7685 * t1841;
    let t38920 = t598 * t1426 * t429 * t9536;
    let t38922 = t137 * t5506;
    let t38925 = t598 * t1426 * t368 * t38922;
    let t38929 = t1980 * t7476 * t38889;
    let t38934 = t598 * t1089 * t22705 * t2090;
    let t38937 = t34406 * t5928;
    (t38916, t38920, t38922, t38925, t38929, t38934, t38937)
}
