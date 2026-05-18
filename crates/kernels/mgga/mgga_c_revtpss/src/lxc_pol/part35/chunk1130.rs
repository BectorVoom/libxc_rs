//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1130/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1130<F: Float>(t94570: F, t7284: F, t96282: F, t26069: F, t96255: F, t2098: F, t4075: F, t786: F, t2103: F, t47567: F, t26261: F, t40270: F) -> (F, F, F, F, F, F) {
    let t96359 = F::new(0.28900264064772933812e-2) * t94570;
    let t96374 = F::new(0.22487184191643109717e-1) * t7284 * t96282;
    let t96401 = F::new(0.91399340044406952588e-2) * t26069 * t96255;
    let t96463 = t786 * t2098 * t4075;
    let t96473 = F::new(0.81814717454467823679e-4) * t47567 * t2103;
    let t96491 = F::new(0.96373646535613327356e-3) * t40270 * t26261;
    (t96359, t96374, t96401, t96463, t96473, t96491)
}
