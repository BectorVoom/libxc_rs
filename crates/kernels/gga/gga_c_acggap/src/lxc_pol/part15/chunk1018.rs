//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1018/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1018<F: Float>(t1809: F, t2020: F, t422: F, t5784: F, t598: F, t599: F, t6: F, t1773: F, t1980: F, t1982: F, t1983: F, t1988: F, t9691: F, t1742: F, t1992: F, t5: F) -> (F, F, F, F, F) {
    let t40147 = t2020 * t1809;
    let t40152 = t598 * t422 * t6 * t5784 * t599;
    let t40156 = t1980 * t1982 * t1773 * t1983;
    let t40158 = t1988 * t9691;
    let t40163 = t1980 * t1982 * t5 * t1742 * t1992;
    (t40147, t40152, t40156, t40158, t40163)
}
