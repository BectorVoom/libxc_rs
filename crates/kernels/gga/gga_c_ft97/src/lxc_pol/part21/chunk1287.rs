//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1287/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1287<F: Float>(t119175: F, t1369: F, t28: F, t586: F, t1882: F, t30228: F, t105810: F, t105816: F, t119978: F, t119982: F, t119985: F, t119988: F, t119992: F, t119996: F, t120000: F, t23649: F, t30179: F) -> (F, F, F, F) {
    let t120004 = t1369 * t28 * t586 * t119175;
    let t120006 = t1882 * t30228;
    let t120007 = 2.0 / 9.0 * t120006;
    let t120008 = t105810 + 3.0 / 4.0 * t119978 - t119982 / 6.0 - t119985 + 2.0 / 3.0 * t119988 + t119992 / 3.0 + 2.0 / 9.0 * t119996 - 12.0 * t120000 - t120004 / 2.0 + t120007 + t105816;
    let t120009 = t23649 * t30179;
    (t120004, t120006, t120008, t120009)
}
