//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 845/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk845<F: Float>(t1952: F, t3413: F, t12633: F, t12637: F, t12665: F, t13030: F, t13136: F, t13180: F, t13228: F, t13230: F, t13234: F, t149: F, t165: F, t3313: F, t614: F) -> F {
    let t13239 = t1952 * t3413;
    let t13245 = -t13228 * t149 - t13234 * t165 - F::new(2.0) * t13239 * t165 - F::new(2.0) * t3313 * t614 - F::new(4.0) * t12633 - F::new(2.0) * t12637 + F::new(4.0) * t12665 - F::new(2.0) * t13030 - F::new(2.0) * t13136 + F::new(8.0) * t13180 + F::new(2.0) * t13230;
    t13245
}
