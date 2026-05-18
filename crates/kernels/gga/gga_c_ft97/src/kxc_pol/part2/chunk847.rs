//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 847/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk847<F: Float>(t13246: F, t184: F, t1064: F, t12092: F, t12244: F, t12253: F, t12257: F, t12261: F, t1577: F, t1580: F, t185: F, t21: F, t2236: F, t2240: F, t2306: F, t3597: F, t3601: F, t363: F, t3660: F, t3668: F, t5: F, t623: F, t920: F) -> F {
    let t13247 = t13246 * t184;
    let t13254 = t623 * t12092 + t623 * t12244 / F::new(4.0) + t3601 * t2306 / F::new(4.0) + t2240 * t3668 / F::new(2.0) + t2240 * t3660 / F::new(2.0) + t623 * t12253 / F::new(2.0) + t623 * t12257 / F::new(2.0) + t623 * t12261 / F::new(4.0) + t5 * t2236 * t920 / F::new(4.0) + t5 * t185 * t1577 / F::new(2.0) + t5 * t3597 * t363 / F::new(2.0) + t5 * t13247 * t21 / F::new(4.0) + t5 * t1064 * t1580 / F::new(4.0);
    t13254
}
