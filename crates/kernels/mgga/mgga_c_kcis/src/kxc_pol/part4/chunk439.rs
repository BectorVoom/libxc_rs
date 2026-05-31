//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 439/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk439<F: Float>(t278: F, t1646: F, t994: F, t993: F, t1697: F) -> (F, F, F) {
    let t288 = F::cast_from(0.0_f64) < t278;
    let t1699 = t994 * t1646;
    let t1700 = t993 * t1699;
    let t1704 = piecewise3::<F>(t288, t1697, -t1697);
    (t1699, t1700, t1704)
}
