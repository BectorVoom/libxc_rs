//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1036/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1036<F: Float>(t1286: F, t1637: F, t5619: F, t1349: F, t5848: F, t5844: F, t24116: F, t5766: F, t5780: F, t5842: F, t7368: F, t1361: F, t7943: F, t23925: F, t378: F, t458: F, t5765: F) -> (F, F, F, F, F, F, F, F, F) {
    let t94049 = t1286 * t1637 * t5619;
    let t94175 = t1349 * t1637 * t5848;
    let t94191 = t1349 * t1637 * t5844;
    let t94198 = t5766 * t24116;
    let t94201 = t1349 * t1637 * t5780;
    let t94208 = t7368 * t5842;
    let t94227 = 14.0 / 81.0 * t1349 * t7943 * t1361;
    let t94230 = t378 * t23925;
    let t94329 = t5765 * t458;
    (t94049, t94175, t94191, t94198, t94201, t94208, t94227, t94230, t94329)
}
