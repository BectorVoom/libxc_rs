//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2211/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2211<F: Float>(t10309: F, t29411: F, t60224: F, t7565: F, t28150: F, t7575: F, t101156: F, t101337: F, t25120: F, t25159: F, t25162: F, t26749: F, t26755: F, t26792: F, t28133: F, t28147: F, t29364: F, t29367: F, t29380: F, t6963: F, t7566: F, t8144: F, t92588: F, t96827: F) -> F {
    let t104203 = t10309 * t29411;
    let t104208 = t60224 * t7565;
    let t104215 = t7575 * t28150;
    let t104222 = t25120 * t8144 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t6963 * t29364 + F::new(2.0) / F::new(3.0) * t6963 * t29367 + F::new(5.0) / F::new(3.0) * t26749 * t28133 - F::new(5.0) * t104203 * t25159 - F::new(5.0) * t26792 * t101337 - F::new(5.0) * t104208 * t25159 - F::new(5.0) / F::new(3.0) * t92588 * t29380 - F::new(10.0) * t96827 * t28147 - F::new(10.0) / F::new(3.0) * t25162 * t104215 + F::new(5.0) / F::new(3.0) * t26755 * t28133 + F::new(5.0) / F::new(3.0) * t7566 * t101156;
    t104222
}
