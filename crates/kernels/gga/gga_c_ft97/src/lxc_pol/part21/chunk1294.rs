//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1294/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1294<F: Float>(t1969: F, t23652: F, t4458: F, t5899: F, t105884: F, t105895: F, t105942: F, t120086: F, t120090: F, t120093: F, t120096: F, t120099: F, t95369: F, t95378: F, t96130: F, t119595: F, t119638: F, t119679: F, t119724: F, t119764: F, t119794: F, t119824: F, t119862: F, t119904: F, t119932: F, t119965: F, t119972: F, t120008: F, t120050: F, t120082: F, t605: F) -> (F, F) {
    let t120103 = t5899 * t1969 * t23652 * t4458;
    let t120105 = t120086 / 4.0 - t120090 - 4.0 / 9.0 * t105884 - t105895 - t96130 + t95369 + t95378 - t105942 + 2.0 * t120093 + 4.0 / 3.0 * t120096 - 4.0 / 9.0 * t120099 - t120103 / 3.0;
    let t120110 = t605 * (t119595 + t119638 + t119679 + t119724 + t119764 + t119794 + t119824 + t119862 + t119904 + t119932 + t119965 + t119972 + t120008 + t120050 + t120082 + t120105);
    (t120103, t120110)
}
