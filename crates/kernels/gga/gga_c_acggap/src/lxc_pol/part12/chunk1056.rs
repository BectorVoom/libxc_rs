//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1056/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1056<F: Float>(t1089: F, t12473: F, t2302: F, t598: F, t3201: F, t8564: F, t1980: F, t7458: F, t8569: F, t1083: F, t137: F, t4875: F) -> (F, F, F, F) {
    let t34775 = t598 * t1089 * t12473 * t2302;
    let t34779 = t598 * t1089 * t3201 * t8564;
    let t34783 = t1980 * t7458 * t3201 * t8569;
    let t34788 = t598 * t1089 * t1083 * t137 * t4875;
    (t34775, t34779, t34783, t34788)
}
