//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 732/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk732<F: Float>(t409: F, t7712: F, t2118: F, t932: F, t2082: F, t7538: F, t1089: F, t2080: F, t429: F, t2079: F, t368: F, t7542: F) -> (F, F, F, F, F, F, F, F) {
    let t7713 = t7712 * t409;
    let t7714 = F::cast_from(0.85748036236139473944e-3_f64) * t7713;
    let t7715 = t2118 * t932;
    let t7717 = t7538 * t2082;
    let t7718 = F::cast_from(0.21437009059034868486e-3_f64) * t7717;
    let t7720 = t1089 * t429 * t2080;
    let t7721 = t2079 * t7720;
    let t7722 = F::cast_from(0.21437009059034868486e-3_f64) * t7721;
    let t7724 = t1089 * t368 * t7542;
    (t7713, t7714, t7715, t7718, t7720, t7721, t7722, t7724)
}
