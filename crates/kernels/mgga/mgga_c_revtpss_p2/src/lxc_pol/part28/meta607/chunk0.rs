//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2104/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2104<F: Float>(t1883: F, t4077: F, t27902: F, t686: F, t72: F, t25878: F, t97732: F, t27840: F, t689: F, t94674: F, t94669: F, t26069: F, t97922: F) -> (F, F, F, F, F, F, F) {
    let t98062 = t1883 * t4077;
    let t98067 = t27902 * t72 * t686;
    let t98069 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t98067;
    let t98071 = F::cast_from(0.51405703062096148812e-1_f64) * t25878 * t97732;
    let t98077 = t27840 * t689;
    let t98078 = t94674 * t98077;
    let t98081 = F::cast_from(0.15421710918628844644e0_f64) * t94669 * t98077;
    let t98084 = t26069 * t97922;
    (t98062, t98067, t98069, t98071, t98078, t98081, t98084)
}
