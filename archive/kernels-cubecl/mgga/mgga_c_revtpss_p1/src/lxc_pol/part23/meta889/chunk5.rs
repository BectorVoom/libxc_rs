//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2824/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2824<F: Float>(t14586: F, t14686: F, t14931: F, t61715: F, t1544: F, t4423: F, t49886: F, t49887: F) -> (F, F, F) {
    let t76362 = t14931 * t14686 * t61715 * t14586;
    let t76372 = t1544 * t4423;
    let t76396 = t49886 + t49887;
    (t76362, t76372, t76396)
}
