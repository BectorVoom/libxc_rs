//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 974/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk974<F: Float>(t10246: F, t10276: F, t10279: F, t10282: F, t10286: F, t10394: F, t10400: F, t14697: F, t14701: F, t14706: F, t15111: F, t14895: F) -> (F, F) {
    let t15112 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14697 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14701 - F::cast_from(2.0_f64) * t14706 + t10394 / F::cast_from(9.0_f64) - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10400 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10276 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10246 - F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t10279 + t10282 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t10286 - t15111;
    let t15116 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t14895;
    (t15112, t15116)
}
