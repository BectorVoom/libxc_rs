//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 950/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk950<F: Float>(t30673: F, t4430: F, t570: F, t1503: F, t7329: F, t1181: F, t2068: F, t22048: F, t604: F, t33751: F, t599: F, t7413: F, t1165: F, t30282: F, t33911: F, t1992: F, t5616: F, t7585: F, t7586: F) -> (F, F, F, F, F, F, F) {
    let t34655 = 0.34299214494455789578e-2 * t30673;
    let t34657 = t570 * t4430;
    let t34659 = t7329 * t1503;
    let t34660 = 7.0 / 72.0 * t34659;
    let t34663 = t2068 * t1181 * t604 * t22048;
    let t34667 = t7413 * t1181 * t599 * t33751;
    let t34671 = t30282 * t1165 * t604 * t33911;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    (t34655, t34657, t34660, t34663, t34667, t34671, t34675)
}
