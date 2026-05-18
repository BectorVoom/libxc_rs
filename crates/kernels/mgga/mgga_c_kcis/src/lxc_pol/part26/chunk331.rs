//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 331/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk331<F: Float>(t1414: F, t1891: F, t1898: F, t1410: F, t1897: F, t456: F) -> (F, F) {
    let t1958 = F::new(0.1982e-1) * t1898 - t1414 - F::new(0.41275e-2) * t1891;
    let t1961 = t1410 * t1897 / F::new(4.0) + t456 * t1958 / F::new(2.0);
    (t1958, t1961)
}
