//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 847/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk847<F: Float>(t150: F, t187: F, t9971: F, t1914: F, t633: F, t8004: F, t1814: F, t7890: F, t944: F, t2385: F, t556: F, t2147: F) -> (F, F, F, F, F, F, F) {
    let t9973 = t9971 * t150 * t187;
    let t9976 = t633 * t1914;
    let t9977 = t8004 * t9976;
    let t9980 = t633 * t1814;
    let t9982 = t7890 * t9980 * t944;
    let t9985 = t2385 * t556;
    let t9986 = t2147 * t9985;
    (t9973, t9976, t9977, t9980, t9982, t9985, t9986)
}
