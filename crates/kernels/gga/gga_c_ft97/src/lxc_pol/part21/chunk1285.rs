//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1285/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1285<F: Float>(t1882: F, t30236: F, t105733: F, t105741: F, t105744: F, t105761: F, t105766: F, t105771: F, t105773: F, t119968: F, t96099: F, t96100: F, t96104: F, t2185: F, t27157: F, t4753: F, t558: F, t5900: F) -> (F, F, F) {
    let t119970 = t1882 * t30236;
    let t119971 = 4.0 / 9.0 * t119970;
    let t119972 = -8.0 / 9.0 * t105733 + t96099 + t96100 - t105741 - t105744 - t105761 + t105766 - t105771 + t105773 - 2.0 / 3.0 * t119968 + t119971 - t96104;
    let t119978 = t27157 * t2185 * t5900 * t4753 * t558;
    (t119970, t119972, t119978)
}
