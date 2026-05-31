//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 591/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk591<F: Float>(t1426: F, t2301: F, t350: F, t4009: F, t4831: F, t4835: F, t4846: F, t974: F, t275: F, t176: F, t1366: F, sigma0: F) -> (F, F, F) {
    let t4848 = -F::cast_from(2.0_f64) * t4009 * t1426 + F::cast_from(2.0_f64) * t2301 * t4835 + t4831 * t350 - t974 * t4846;
    let t4849 = t4848 * t275;
    let t4851 = t176 * t4849 * sigma0;
    let t4854 = t1366 * t1366;
    (t4848, t4851, t4854)
}
