//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1059/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1059<F: Float>(t1181: F, t4526: F, t7351: F, t7564: F, t4680: F, t7426: F, t8476: F, t7575: F, t8445: F, t30937: F, t8450: F, t31346: F, t4269: F) -> (F, F, F, F, F) {
    let t34553 = t7564 * t1181 * t7351 * t4526;
    let t34556 = t7426 * t4680 * t8476;
    let t34557 = F::new(0.62896184579208304136e-3) * t34556;
    let t34559 = t7575 * t4680 * t8445;
    let t34561 = t30937 * t8450;
    let t34562 = F::new(0.18868855373762491241e-2) * t34561;
    let t34563 = t31346 * t4269;
    (t34553, t34557, t34559, t34562, t34563)
}
