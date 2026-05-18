//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 922/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk922<F: Float>(t2469: F, t3842: F, t729: F, t2579: F, t3977: F, t1882: F, t3856: F, t3974: F, t3972: F, t242: F, t10126: F, t10128: F, t10134: F, t10140: F, t10146: F, t10148: F, t14256: F, t14261: F, t14265: F, t14269: F, t446: F) -> (F, F) {
    let t14273 = t729 * t2469 * t3842;
    let t14277 = t729 * t3977 * t2579;
    let t14281 = F::new(2.0) / F::new(27.0) * t1882 * t3856;
    let t14283 = F::new(2.0) / F::new(9.0) * t1882 * t3974;
    let t14288 = t2469 * t3972;
    let t14289 = t242 * t14288;
    let t14292 = t10126 / F::new(27.0) + F::new(2.0) / F::new(81.0) * t10128 + F::new(2.0) / F::new(3.0) * t446 * t14256 + t446 * t14261 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t446 * t14265 + F::new(4.0) / F::new(3.0) * t446 * t14269 + F::new(2.0) / F::new(3.0) * t446 * t14273 + F::new(2.0) / F::new(3.0) * t446 * t14277 + t14281 + t14283 - F::new(8.0) / F::new(81.0) * t10134 + F::new(2.0) / F::new(27.0) * t10140 - F::new(2.0) / F::new(27.0) * t10146 - F::new(2.0) / F::new(9.0) * t10148 - F::new(2.0) / F::new(3.0) * t446 * t14289;
    (t14288, t14292)
}
