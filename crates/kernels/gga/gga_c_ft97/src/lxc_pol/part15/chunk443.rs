//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 443/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk443<F: Float>(t348: F, t4500: F, t965: F, t1766: F, t91: F, t1781: F, t4417: F, t1780: F, t1787: F, t4422: F, t1791: F, t463: F) -> (F, F, F, F, F, F, F, F) {
    let t4501 = t348 * t4500;
    let t4505 = t965 * t965;
    let t4507 = t91 * t1766 * t4505;
    let t4511 = t1781 * t4417;
    let t4512 = t1780 * t4511;
    let t4515 = t1787 * t4422;
    let t4518 = t1791 * t4417;
    let t4519 = t463 * t4518;
    (t4501, t4505, t4507, t4511, t4512, t4515, t4518, t4519)
}
