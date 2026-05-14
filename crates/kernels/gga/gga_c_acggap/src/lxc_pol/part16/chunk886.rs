//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 886/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk886<F: Float>(t1998: F, t4625: F, t1434: F, t7736: F, t1418: F, t7614: F, t1083: F, t1487: F, t1980: F, t355: F, t7458: F, t7799: F, t8571: F, t3201: F, t8569: F, t1988: F, t8549: F) -> (F, F, F, F, F, F, F) {
    let t34745 = t1998 * t4625;
    let t34746 = 0.17149607247227894789e-2 * t34745;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34754 = 0.32012600194825403606e-1 * t34753;
    let t34767 = t1980 * t7458 * t1083 * t355 * t1487;
    let t34771 = t7799 * t8571;
    let t34783 = t1980 * t7458 * t3201 * t8569;
    let t34794 = t1988 * t8549;
    (t34746, t34751, t34754, t34767, t34771, t34783, t34794)
}
