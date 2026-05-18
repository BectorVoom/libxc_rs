//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 404/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk404<F: Float>(t160: F, t2133: F, t149: F, t165: F, t1953: F, t2081: F, t2143: F, t2158: F, t2181: F, t2228: F, t564: F, t614: F) -> (F, F) {
    let t2230 = t2133 * t160;
    let t2235 = -t149 * t2228 - t165 * t1953 - t165 * t2081 - F::new(2.0) * t564 * t614 - F::new(4.0) * t2143 - F::new(2.0) * t2158 + F::new(4.0) * t2181 + F::new(2.0) * t2230;
    (t2230, t2235)
}
