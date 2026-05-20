//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2041/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2041<F: Float>(t25878: F, t98067: F, t97732: F, t27840: F, t689: F, t94674: F, t94669: F, t26069: F, t97922: F, t28011: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t98069 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t98067;
    let t98071 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t97732;
    let t98077 = t27840 * t689;
    let t98078 = t94674 * t98077;
    let t98081 = F::cast_from(0.15421710918628844644e0_f64) * t94669 * t98077;
    let t98084 = t26069 * t97922;
    let t98087 = t28011 * t72 * t686;
    (t98069, t98071, t98078, t98081, t98084, t98087)
}
