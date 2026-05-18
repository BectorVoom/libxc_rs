//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 46/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk46<F: Float>(t55: F, t58: F, t61: F, t69: F) -> (F, F, F) {
    let t106 = F::new(0.705945e1) * t58 + F::new(0.1549425e1) * t55 + F::new(0.420775e0) * t61 + F::new(0.1562925e0) * t69;
    let t109 = F::new(1.0) + F::new(0.32163958997385070134e2) / t106;
    let t110 = f64::ln(t109);
    (t106, t109, t110)
}
