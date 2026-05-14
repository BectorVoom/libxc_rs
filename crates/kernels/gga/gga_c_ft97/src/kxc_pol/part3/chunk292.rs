//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 292/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk292<F: Float>(t292: F, t1208: F, t817: F, t1111: F, t1198: F, t1201: F, t285: F) -> (F,) {
    let t293 = 0.1e-59 < t292;
    let t1209 = t817 * t1208;
    let t1212 = piecewise3(t293, 2.0 * t1198 - 0.60409133884038297798e0 * t1201 * t1111 + 0.60409133884038297798e0 * t292 * t1111 - t285 * t1209, 0.0);
    (t1212,)
}
