//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1053/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1053<F: Float>(t11176: F, t1402: F, t28012: F, t107954: F, t107958: F, t107964: F, t107966: F, t107968: F, t107971: F, t107976: F, t13852: F, t14133: F, t24204: F, t24232: F, t28027: F, t28033: F, t42500: F, t6002: F, t6003: F, t96818: F, t98168: F) -> (F,) {
    let t107979 = t1402 * t11176 * t28012;
    let t107982 = -t6002 * t98168 * t24232 * t13852 / 3.0 - 2.0 * t107954 - t107958 - 4.0 * t6002 * t42500 * t6003 * t14133 + 8.0 * t107964 + 8.0 * t107966 + 8.0 * t107968 + t107971 + 2.0 / 9.0 * t24204 * t28027 + 2.0 / 9.0 * t24204 * t28033 - 2.0 * t107976 - 11.0 / 27.0 * t107979 - 8.0 / 27.0 * t96818;
    (t107982,)
}
