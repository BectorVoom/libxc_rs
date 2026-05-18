//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1339/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1339<F: Float>(t32436: F, t8709: F, t8710: F, t8711: F, t8713: F, t8715: F, t9128: F, t9129: F, t9744: F, t9746: F, t9748: F) -> F {
    let tv4rho43 = F::new(3.0) * t8709 + F::new(3.0) * t8710 + F::new(6.0) * t8711 + F::new(6.0) * t8713 + F::new(3.0) * t8715 + F::new(3.0) * t9128 + F::new(0.1434375e0) * t9129 - F::new(0.7171875e-1) * t9744 - F::new(0.4303125e0) * t9746 + F::new(0.286875e0) * t9748 + t32436;
    tv4rho43
}
