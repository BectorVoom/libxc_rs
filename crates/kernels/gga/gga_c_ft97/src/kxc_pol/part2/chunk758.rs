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
    let t11913 = F::new(2.0) / F::new(27.0) * t8392 * t3184;
    let t11922 = F::new(2.0) / F::new(9.0) * t10992;
    let t11928 = -t8437 + F::new(4.0) / F::new(9.0) * t10976 + F::new(2.0) / F::new(3.0) * t10981 + t10985 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t10990 - t11922 + F::new(4.0) / F::new(3.0) * t10996 + F::new(2.0) / F::new(3.0) * t11000 + F::new(8.0) / F::new(3.0) * t11005 - F::new(10.0) / F::new(27.0) * t11010 - F::new(8.0) / F::new(9.0) * t11015;
    (t11913, t11928)
}
