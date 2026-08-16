//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1136/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1136<F: Float>(t1323: F, t507: F, t7436: F, t1181: F, t30806: F, t5824: F, t599: F, t5969: F, t7493: F, t1839: F, t1983: F, t7585: F, t7586: F) -> (F, F, F, F) {
    let t39661 = t7436 * t507 * t1323;
    let t39665 = t30806 * t1181 * t599 * t5824;
    let t39669 = t7493 * t1181 * t599 * t5969;
    let t39673 = t7585 * t7586 * t1983 * t1839;
    (t39661, t39665, t39669, t39673)
}
