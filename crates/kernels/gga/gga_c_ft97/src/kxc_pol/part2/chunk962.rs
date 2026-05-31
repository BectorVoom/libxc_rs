//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 962/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk962<F: Float>(t10279: F, t10400: F, t10552: F, t10555: F, t10636: F, t10641: F, t10643: F, t14697: F, t14701: F, t14706: F, t14946: F, t14895: F) -> (F, F) {
    let t14947 = F::cast_from(4.0_f64) * t14697 + F::cast_from(2.0_f64) * t14701 - F::cast_from(6.0_f64) * t14706 + t10552 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t10400 - t10555 - t10636 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10279 + t10641 + t10643 - t14946;
    let t14949 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14895;
    (t14947, t14949)
}
