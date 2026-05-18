//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1034/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1034<F: Float>(t1988: F, t8566: F, t2001: F, t4754: F, t4759: F, t4764: F, t4456: F, t8511: F, t1165: F, t4665: F, t7564: F, t8600: F) -> (F, F, F, F, F, F) {
    let t34221 = t1988 * t8566;
    let t34222 = F::new(0.62896184579208304136e-3) * t34221;
    let t34223 = t2001 * t4754;
    let t34225 = t2001 * t4759;
    let t34227 = t2001 * t4764;
    let t34229 = t8511 * t4456;
    let t34233 = t7564 * t1165 * t8600 * t4665;
    (t34222, t34223, t34225, t34227, t34229, t34233)
}
