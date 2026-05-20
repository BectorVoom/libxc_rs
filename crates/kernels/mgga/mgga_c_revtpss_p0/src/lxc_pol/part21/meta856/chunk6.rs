//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3253/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3253<F: Float>(t10298: F, t10309: F, t10310: F, t10410: F, t13269: F, t13283: F, t13420: F, t1497: F, t2242: F, t2247: F, t2315: F, t4173: F, t4178: F, t4241: F, t45955: F, t45963: F, t45972: F, t60248: F, t603: F, t60360: F, t60391: F, t60417: F, t60483: F, t644: F) -> F {
    let t60496 = F::new(20.0) * t2247 * t1497 * t10410 - F::new(12.0) * t60248 * t644 - F::new(12.0) * t13269 * t2315 - F::new(4.0) * t4173 * t10410 - F::new(4.0) * t45955 * t1497 - F::new(12.0) * t10298 * t4241 - F::new(12.0) * t2242 * t13420 - F::new(4.0) * t603 * (t60360 + t60391 + t60417 + t60483) - F::new(360.0) * t45963 * t13283 + F::new(840.0) * t45972 * t1497 * t10310 - F::new(360.0) * t10309 * t4178 * t2315;
    t60496
}
