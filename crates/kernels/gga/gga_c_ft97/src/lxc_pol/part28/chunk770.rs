//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 770/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk770<F: Float>(t32457: F, t488: F, t83: F, t1882: F, t7226: F, t432: F, t452: F, t7288: F, t7283: F, t1825: F, t7281: F, t7274: F, t8466: F) -> (F, F, F, F, F, F, F, F) {
    let t32458 = t488 * t32457;
    let t32459 = t83 * t32458;
    let t32463 = F::new(2.0) / F::new(9.0) * t1882 * t7226;
    let t32465 = t452 * t7288 * t432;
    let t32469 = t1882 * t7283 / F::new(9.0);
    let t32470 = t1825 * t7281;
    let t32471 = t83 * t32470;
    let t32474 = t8466 * t7274;
    (t32458, t32459, t32463, t32465, t32469, t32470, t32471, t32474)
}
