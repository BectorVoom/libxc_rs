//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 608/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk608<F: Float>(t1117: F, t883: F, t1125: F, t972: F, t3274: F, t3276: F, t3279: F, t3282: F, t3286: F, t3290: F, t3298: F, t3301: F, t3305: F, t3308: F, t3310: F) -> (F, F, F) {
    let t3565 = t1117 * t883;
    let t3568 = t1125 * t972;
    let t3582 = -F::new(0.3373480902777777778e-5) * t3274 - F::new(0.16908181191593721013e-4) * t3276 + F::new(0.14492726735651760868e-5) * t3279 + F::new(0.12357942809624928455e-3) * t3282 + F::new(0.28985453471303521736e-5) * t3286 - F::new(0.28985453471303521736e-5) * t3290 + F::new(0.14758978949652777779e-5) * t3298 - F::new(0.50680539737635041235e-4) * t3301 - F::new(0.14492726735651760868e-5) * t3305 + F::new(0.27801896084645508334e-2) * t3308 + F::new(0.27801896084645508334e-2) * t3310;
    (t3565, t3568, t3582)
}
