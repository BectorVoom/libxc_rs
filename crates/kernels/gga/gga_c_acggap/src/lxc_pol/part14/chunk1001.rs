//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1001/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1001<F: Float>(t30811: F, t6347: F, t142: F, t2060: F, t5674: F, t604: F, t1323: F, t507: F, t7436: F, t1181: F, t30806: F, t5824: F, t599: F, t5969: F, t7493: F, t1839: F, t1983: F, t7585: F, t7586: F) -> (F, F, F, F, F, F) {
    let t39653 = t30811 * t6347;
    let t39658 = t2060 * t142 * t604 * t5674;
    let t39661 = t7436 * t507 * t1323;
    let t39665 = t30806 * t1181 * t599 * t5824;
    let t39669 = t7493 * t1181 * t599 * t5969;
    let t39673 = t7585 * t7586 * t1983 * t1839;
    (t39653, t39658, t39661, t39665, t39669, t39673)
}
