//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1152/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1152<F: Float>(t1882: F, t28175: F, t668: F, t880: F, t1477: F, t9568: F, t317: F, t9570: F, t25462: F, t28947: F, t28960: F, t6210: F, t1466: F, t29016: F, t681: F, t7021: F) -> (F, F, F, F, F, F, F, F) {
    let t111530 = 2.0 / 9.0 * t1882 * t28175;
    let t111592 = t880 * t668;
    let t111624 = t9568 * t1477;
    let t111625 = t317 * t9570;
    let t111657 = 2.0 / 81.0 * t25462 * t28947;
    let t111664 = t6210 * t28960 / 9.0;
    let t111667 = t1466 * t681 * t29016 / 9.0;
    let t111668 = t7021 * t880;
    (t111530, t111592, t111624, t111625, t111657, t111664, t111667, t111668)
}
