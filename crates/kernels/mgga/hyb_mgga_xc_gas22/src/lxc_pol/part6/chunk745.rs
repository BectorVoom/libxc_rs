//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 745/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk745<F: Float>(t2111: F, t4051: F, t2132: F, t173: F, t178: F, t180: F, t181: F, t2124: F, t4046: F, t747: F, t751: F, t1270: F, t1282: F, t172: F, t184: F, t2116: F, t742: F) -> (F, F, F, F) {
    let t4052 = t2111 * t4051;
    let t4068 = t2132 * t4051;
    let t4079 = -2.0 * t2124 * t4051 * t180 + t747 * t4046 * t180 / 2.0 + t4068 * t180 / 4.0 - 4.0 * t4051 * t181 - t178 * t4051 * t180 - 4.0 * t751 * t4046 - t173 * t4046 * t180;
    let t4082 = -t4052 * t180 / 2.0 + 2.0 * t2116 * t4051 - t742 * t4046 + 2.0 * t4046 * t184 + 4.0 * t1270 * t1282 + 2.0 * t172 * t4079;
    (t4052, t4068, t4079, t4082)
}
