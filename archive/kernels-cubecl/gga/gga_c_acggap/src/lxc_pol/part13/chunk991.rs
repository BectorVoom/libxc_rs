//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 991/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk991<F: Float>(t2132: F, t322: F, t7896: F, t8422: F, t556: F, t943: F, t944: F, t880: F, t9062: F, t157: F, t929: F, t1960: F, t5368: F) -> (F, F, F, F, F, F) {
    let t33635 = t7896 * t2132 * t8422 * t322;
    let t33643 = t556 * t943;
    let t33644 = t33643 * t944;
    let t33648 = t9062 * t880;
    let t33651 = t556 * t929 * t157;
    let t33656 = t1960 * t5368;
    (t33635, t33643, t33644, t33648, t33651, t33656)
}
