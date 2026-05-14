//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 391/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk391<F: Float>(t1766: F, t4505: F, t91: F, t1781: F, t4417: F, t1780: F, t1787: F, t4422: F, t1791: F, t463: F, t4431: F, t464: F, t1800: F, t24: F, t4436: F, t4495: F, t469: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4507 = t91 * t1766 * t4505;
    let t4511 = t1781 * t4417;
    let t4512 = t1780 * t4511;
    let t4515 = t1787 * t4422;
    let t4518 = t1791 * t4417;
    let t4519 = t463 * t4518;
    let t4522 = t464 * t4431;
    let t4523 = t463 * t4522;
    let t4527 = t24 * t1800 * t4436;
    let t4531 = t24 * t469 * t4495;
    (t4507, t4511, t4512, t4515, t4518, t4519, t4522, t4523, t4527, t4531)
}
