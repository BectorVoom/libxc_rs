//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1068/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1068<F: Float>(t30673: F, t4430: F, t570: F, t1503: F, t7329: F, t1181: F, t2068: F, t22048: F, t604: F, t33751: F, t599: F, t7413: F) -> (F, F, F, F, F) {
    let t34655 = F::cast_from(0.34299214494455789578e-2_f64) * t30673;
    let t34657 = t570 * t4430;
    let t34659 = t7329 * t1503;
    let t34660 = F::new(7.0) / F::new(72.0) * t34659;
    let t34663 = t2068 * t1181 * t604 * t22048;
    let t34667 = t7413 * t1181 * t599 * t33751;
    (t34655, t34657, t34660, t34663, t34667)
}
