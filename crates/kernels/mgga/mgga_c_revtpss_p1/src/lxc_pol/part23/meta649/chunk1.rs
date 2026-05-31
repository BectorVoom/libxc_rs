//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2375/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2375<F: Float>(t2237: F, t2482: F, t823: F, t2487: F, t10111: F, t849: F, t9720: F, t685: F, t775: F, t855: F, t242: F, t240: F, t72: F) -> (F, F, F, F, F) {
    let t40424 = t2482 * t823 * t2237;
    let t40425 = t40424 * t2487;
    let t40452 = t10111 * t849 * t9720;
    let t40455 = t40452 * t855 * t685 * t775;
    let t40459 = t242 * t242;
    let t40460 = F::cast_from(1.0_f64) / t40459;
    let t40462 = t240 * t40460 * t72;
    (t40424, t40425, t40452, t40455, t40462)
}
