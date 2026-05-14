//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 435/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk435<F: Float>(t1781: F, t4417: F, t1780: F, t1787: F, t4422: F, t1791: F, t463: F, t4431: F, t464: F, t1800: F, t24: F, t4436: F, t4495: F, t469: F, t1773: F, t3125: F, t3144: F, t462: F, t92: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4511 = t1781 * t4417;
    let t4512 = t1780 * t4511;
    let t4515 = t1787 * t4422;
    let t4518 = t1791 * t4417;
    let t4519 = t463 * t4518;
    let t4522 = t464 * t4431;
    let t4523 = t463 * t4522;
    let t4527 = t24 * t1800 * t4436;
    let t4531 = t24 * t469 * t4495;
    let t4533 = t1773 + 2.0 / 9.0 * t3125 + 2.0 / 3.0 * t3144 - 2.0 / 9.0 * t462 * t4512 + 2.0 / 3.0 * t462 * t4515 + 2.0 / 3.0 * t462 * t4519 - t462 * t4523 / 3.0 + 2.0 * t92 * t4527 - t92 * t4531;
    (t4511, t4512, t4515, t4518, t4519, t4522, t4523, t4527, t4531, t4533)
}
