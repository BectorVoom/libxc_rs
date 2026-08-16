//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 294/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk294<F: Float>(t1137: F, t1169: F, t1173: F, t1175: F, t247: F, t263: F, t792: F, t992: F, t666: F, t89: F, t1095: F, t801: F) -> (F, F, F, F) {
    let t1178 = -t1137 * t263 - t1173 * t247 - F::cast_from(2.0_f64) * t1169 + F::cast_from(2.0_f64) * t1175;
    let t1186 = t792 * t992;
    let t1188 = t89 * t666 * t1186;
    let t1190 = t801 * t1095;
    (t1178, t1186, t1188, t1190)
}
