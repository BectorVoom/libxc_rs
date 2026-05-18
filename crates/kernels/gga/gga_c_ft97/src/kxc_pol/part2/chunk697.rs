//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 697/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk697<F: Float>(t8192: F, t10976: F, t10981: F, t10985: F, t10990: F, t10993: F, t10996: F, t11000: F, t11005: F, t11010: F, t11015: F, t11019: F, t11022: F, t11024: F, t11026: F, t11027: F, t7778: F, t7782: F, t7820: F, t7822: F) -> F {
    let t11031 = F::new(4.0) / F::new(27.0) * t8192;
    let t11032 = -t7822 / F::new(27.0) + F::new(2.0) / F::new(27.0) * t10976 + t10981 / F::new(9.0) + t10985 / F::new(18.0) + t10990 / F::new(27.0) - t10993 + F::new(2.0) / F::new(9.0) * t10996 + t11000 / F::new(9.0) + F::new(4.0) / F::new(9.0) * t11005 - F::new(5.0) / F::new(81.0) * t11010 - F::new(4.0) / F::new(27.0) * t11015 + t11019 / F::new(18.0) - t11022 - t11024 + t11026 - t11027 + t7778 / F::new(54.0) + t7782 / F::new(81.0) - t7820 / F::new(27.0) - t11031;
    t11032
}
