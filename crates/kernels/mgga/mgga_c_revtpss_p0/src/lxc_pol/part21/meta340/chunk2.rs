//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1662/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1662<F: Float>(t11467: F, t3014: F, t11132: F, t11337: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11339: F, t11343: F, t11346: F) -> (F, F, F, F) {
    let t11468 = t11467 * t3014;
    let t11479 = F::cast_from(0.93932222222222222223e0_f64) * t11132;
    let t11480 = F::cast_from(0.36793333333333333333e0_f64) * t11337;
    let t11485 = F::new(0.16504875e0) * t11316 - F::new(0.82785e-1) * t11319 + F::new(0.49671e0) * t11322 + F::new(0.181155e1) * t11167 - F::cast_from(0.60384999999999999999e0_f64) * t11158 - F::new(0.33114e0) * t11326 + F::new(0.16557e0) * t11329 - F::new(0.49671e0) * t11332 - t11479 - t11480 + F::new(0.5519e-1) * t11339 - F::cast_from(0.36793333333333333333e-1_f64) * t11343 - F::new(0.82785e-1) * t11346 - F::new(0.181155e1) * t11162;
    (t11468, t11479, t11480, t11485)
}
