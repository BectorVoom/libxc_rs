//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 411/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk411<F: Float>(t149: F, t165: F, t1953: F, t2081: F, t2143: F, t2158: F, t2181: F, t2228: F, t2230: F, t564: F, t614: F, t184: F) -> (F, F) {
    let t2235 = -t149 * t2228 - t165 * t1953 - t165 * t2081 - F::new(2.0) * t564 * t614 - F::new(4.0) * t2143 - F::new(2.0) * t2158 + F::new(4.0) * t2181 + F::new(2.0) * t2230;
    let t2236 = t2235 * t184;
    (t2235, t2236)
}
