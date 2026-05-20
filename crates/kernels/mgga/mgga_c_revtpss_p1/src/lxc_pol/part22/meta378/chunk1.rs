//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1933/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1933<F: Float>(t13363: F, t13419: F, t10298: F, t10301: F, t10309: F, t13267: F, t13269: F, t13272: F, t13283: F, t13286: F, t13289: F, t1497: F, t2242: F, t2247: F, t2248: F, t2315: F, t4173: F, t4178: F, t4241: F, t603: F, t644: F, t91: F) -> (F, F) {
    let t13420 = t13363 + t13419;
    let t13423 = -F::new(4.0) * t10298 * t1497 + F::new(40.0) * t10301 * t4178 - F::new(120.0) * t10309 * t13283 + t13267 * t91 - F::new(8.0) * t13269 * t644 + F::new(20.0) * t13272 * t2248 + F::new(40.0) * t13286 * t2247 + F::new(20.0) * t13289 * t2247 - F::new(4.0) * t13420 * t603 - F::new(8.0) * t2242 * t4241 - F::new(4.0) * t2315 * t4173;
    (t13420, t13423)
}
