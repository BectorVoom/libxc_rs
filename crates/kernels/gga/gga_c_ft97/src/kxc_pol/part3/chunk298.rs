//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 298/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk298<F: Float>(t1212: F, t799: F, t27: F, t89: F, t1188: F, t791: F, t788: F) -> (F, F, F, F) {
    let t1213 = t799 * t1212;
    let t1215 = t89 * t27 * t1213;
    let t1217 = -t791 - t1188 / F::cast_from(18.0_f64) - t1215 / F::cast_from(6.0_f64);
    let t1218 = t788 * t1217;
    (t1213, t1215, t1217, t1218)
}
