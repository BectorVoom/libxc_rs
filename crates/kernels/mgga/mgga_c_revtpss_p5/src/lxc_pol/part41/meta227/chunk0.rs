//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 884/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk884<F: Float>(t2880: F, t6113: F, t2884: F, t4571: F, t6094: F, t6098: F, t6102: F, t916: F, t2897: F, t923: F, t2908: F, t6092: F) -> (F, F, F, F, F, F) {
    let t6114 = t2880 * t6113;
    let t6120 = t2884 + F::new(2.0) / F::new(9.0) * t4571 - F::new(2.0) / F::new(9.0) * t6094 + F::new(2.0) / F::new(3.0) * t6098 - t6102 / F::new(3.0);
    let t6121 = t916 * t6120;
    let t6127 = t2897 * t6113;
    let t6129 = t923 * t6120;
    let t6132 = t2908 * t6092;
    (t6114, t6120, t6121, t6127, t6129, t6132)
}
