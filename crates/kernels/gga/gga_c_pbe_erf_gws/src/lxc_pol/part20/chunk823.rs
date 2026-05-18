//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 823/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk823<F: Float>(t34: F, t597: F, t1033: F, t1683: F, t2749: F, t633: F, t219: F, t641: F, t1639: F, t5219: F, t995: F, t5212: F, t626: F) -> (F, F, F, F, F, F, F) {
    let t7468 = t597 * t34;
    let t7474 = F::new(8.0) / F::new(45.0) * t1033 * t1683;
    let t7478 = F::new(8.0) / F::new(45.0) * t633 * t2749;
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7495 = t5219 * t995;
    let t7499 = t5212 * t626;
    (t7468, t7474, t7478, t7483, t7490, t7495, t7499)
}
