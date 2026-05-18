//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 894/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk894<F: Float>(t27232: F, t3145: F, t8045: F, t9256: F, t12856: F, t17293: F, t605: F, t1382: F, t2497: F, t3418: F, t32100: F, t921: F) -> (F, F, F, F, F) {
    let t42503 = F::new(2.0) * t27232 * t3145;
    let t42506 = F::new(4.0) * t8045 * t9256;
    let t42509 = F::new(24.0) * t17293 * t12856 * t605;
    let t42511 = t1382 * t3418 * t2497;
    let t42512 = F::new(4.0) * t42511;
    let t42513 = t32100 * t921;
    (t42503, t42506, t42509, t42512, t42513)
}
