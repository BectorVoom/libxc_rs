//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1100/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1100<F: Float>(t173: F, t184: F, t199: F, t47598: F, t47611: F, t30856: F, t40474: F, t3429: F, t3454: F, t5548: F, t587: F, t1815: F, t40396: F, t639: F, t954: F) -> (F, F, F, F, F) {
    let t47616 = F::new(2.0) / F::new(15.0) * t173 * (t47598 + t47611) * t184 * t199;
    let t47617 = F::new(16.0) / F::new(81.0) * t30856;
    let t47618 = F::new(64.0) / F::new(27.0) * t40474;
    let t47622 = F::new(16.0) / F::new(15.0) * t587 * t5548 * t3429 * t3454;
    let t47626 = F::new(16.0) / F::new(45.0) * t639 * t1815 * t40396 * t954;
    (t47616, t47617, t47618, t47622, t47626)
}
