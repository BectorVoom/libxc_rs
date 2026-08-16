//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1841/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1841<F: Float>(t3053: F, t6765: F, t3127: F, t3037: F, t3033: F, sigma0: F) -> (F, F, F, F) {
    let t23533 = t6765 * t3053;
    let t23535 = t3127 * sigma0;
    let t23536 = t23535 * t3037;
    let t23537 = t3033 * t23536;
    (t23533, t23535, t23536, t23537)
}
