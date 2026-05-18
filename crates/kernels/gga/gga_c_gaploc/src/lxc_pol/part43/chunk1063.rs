//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1063/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1063<F: Float>(t41340: F, t43895: F, t43908: F, t43909: F, t43910: F, t43913: F, t43915: F, t43918: F, t43924: F, t43925: F, t43927: F, t43931: F, t43935: F, t47412: F, t47415: F, t47417: F, t47419: F, t47423: F, t47430: F, t47432: F) -> F {
    let t51152 = -t47412 - t47415 + t47417 - t47419 - t47423 - t43895 - t47430 + t43908 + F::new(0.47667319935800568892e0) * t47432 - t43909 + t43910 + F::new(0.89376224879626066675e-1) * t41340 - t43913 + t43915 + t43918 + t43924 - F::new(0.89376224879626066674e-1) * t43925 - F::new(0.89376224879626066674e-1) * t43927 - t43931 - t43935;
    t51152
}
