//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 758/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk758<F: Float>(t3184: F, t8392: F, t10992: F, t10976: F, t10981: F, t10985: F, t10990: F, t10996: F, t11000: F, t11005: F, t11010: F, t11015: F, t8437: F) -> (F, F) {
    let t11913 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3184;
    let t11922 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10992;
    let t11928 = -t8437 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10976 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10981 + t10985 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10990 - t11922 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10996 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t11000 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11005 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t11010 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11015;
    (t11913, t11928)
}
