//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 891/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk891<F: Float>(t1181: F, t21118: F, t7351: F, t7426: F, t1165: F, t21955: F, t30806: F, t604: F, t30924: F, t30928: F, t1164: F, t8853: F, t31142: F, t8884: F, t2019: F, t8887: F, t8889: F) -> (F, F, F, F, F, F, F) {
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35113 = t30806 * t1165 * t604 * t21955;
    let t35123 = 0.75475421495049964964e-2 * t30924;
    let t35125 = 0.75475421495049964964e-2 * t30928;
    let t35137 = t1164 * t8853;
    let t35145 = t31142 * t8884;
    let t35148 = t2019 * t8887 * t8889;
    (t35100, t35113, t35123, t35125, t35137, t35145, t35148)
}
