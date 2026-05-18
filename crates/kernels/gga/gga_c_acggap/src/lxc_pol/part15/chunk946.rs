//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 946/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk946<F: Float>(t2132: F, t2138: F, t3101: F, t633: F, t3645: F, t635: F, t8114: F, t880: F, t323: F, t3242: F, t309: F, t8306: F) -> (F, F, F, F, F) {
    let t33185 = F::new(0.8673628188205199462e0) * t2138 * t2132 * t633 * t3101;
    let t33201 = F::new(0.65854491829355115987e0) * t3645 * t635;
    let t33208 = t8114 * t880;
    let t33227 = F::new(0.19756347548806534796e1) * t3242 * t633 * t323;
    let t33232 = t8306 * t309;
    (t33185, t33201, t33208, t33227, t33232)
}
