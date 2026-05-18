//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 986/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk986<F: Float>(t33175: F, t7963: F, t7965: F, t2132: F, t2138: F, t3101: F, t633: F, t4210: F, t7942: F, t3645: F, t635: F, t8114: F, t880: F) -> (F, F, F, F, F) {
    let t33180 = t7963 * t33175 * t7965;
    let t33185 = F::new(0.8673628188205199462e0) * t2138 * t2132 * t633 * t3101;
    let t33198 = t7942 * t33175 * t4210;
    let t33201 = F::new(0.65854491829355115987e0) * t3645 * t635;
    let t33208 = t8114 * t880;
    (t33180, t33185, t33198, t33201, t33208)
}
