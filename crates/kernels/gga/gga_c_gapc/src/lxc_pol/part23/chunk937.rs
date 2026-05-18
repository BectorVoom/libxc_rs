//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 937/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk937<F: Float>(t1036: F, t5121: F, t11488: F, t1688: F, t5126: F, t11320: F, t185: F, t1697: F, t3122: F) -> (F, F, F, F, F, F) {
    let t11489 = t1036 * t5121;
    let t11490 = t11488 * t11489;
    let t11492 = t1688 * t5126;
    let t11493 = t11488 * t11492;
    let t11495 = t185 * t11320;
    let t11496 = t1697 * t3122;
    (t11489, t11490, t11492, t11493, t11495, t11496)
}
