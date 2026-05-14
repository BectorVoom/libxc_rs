//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 700/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk700<F: Float>(t1588: F, t1871: F, t986: F, t10970: F, t83: F, t1651: F, t447: F, t1643: F, t1866: F, t3206: F, t8392: F, t1580: F, t920: F) -> (F, F, F, F, F, F) {
    let t11966 = t1871 * t986 * t1588;
    let t11969 = t83 * t10970;
    let t11973 = t447 * t986 * t1651;
    let t11977 = t1866 * t986 * t1643;
    let t11981 = 2.0 / 27.0 * t8392 * t3206;
    let t11982 = t920 * t1580;
    (t11966, t11969, t11973, t11977, t11981, t11982)
}
