//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 727/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk727<F: Float>(t1025: F, t1028: F, t1041: F, t1047: F, t1665: F, t1671: F, t3124: F, t3127: F, t3194: F, t3203: F, t3211: F, t3216: F, t3224: F, t4854: F, t4858: F, t4869: F, t4875: F, t4879: F) -> F {
    let t4883 = -F::new(0.21437009059034868486e-3) * t3224 * t1665 - F::new(0.21437009059034868486e-3) * t1025 * t4854 - F::new(0.21437009059034868486e-3) * t4858 * t1028 + F::new(0.11433071498151929859e-2) * t3211 * t1665 + F::new(0.14291339372689912324e-3) * t3194 - t3203 + F::new(0.21437009059034868486e-3) * t3124 * t1671 + F::new(0.21437009059034868486e-3) * t1041 * t4869 - F::new(0.14291339372689912324e-3) * t3127 * t4875 + F::new(0.21437009059034868486e-3) * t4879 * t1047 - F::new(0.14291339372689912324e-3) * t3216;
    t4883
}
