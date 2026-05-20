//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1340/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1340<F: Float>(t10111: F, t849: F, t9720: F, t242: F, t240: F, t72: F, t212: F, t2237: F, t225: F, t816: F, t10689: F, t237: F, t247: F) -> (F, F, F, F) {
    let t40452 = t10111 * t849 * t9720;
    let t40459 = t242 * t242;
    let t40460 = F::new(1.0) / t40459;
    let t40462 = t240 * t40460 * t72;
    let t40488 = t816 * t2237 * t212 * t225;
    let t40507 = F::cast_from(0.28974367305964659283e0_f64) * t237 * t10689 * t247;
    (t40452, t40462, t40488, t40507)
}
