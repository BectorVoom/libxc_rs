//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1149/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1149<F: Float>(t15573: F, t2173: F, t28957: F, t28951: F, t1003: F, t100575: F, t100578: F, t100580: F, t100583: F, t100586: F, t100975: F, t27772: F, t28948: F, t28952: F, t7687: F, t7696: F, t7703: F, t96019: F) -> (F, F, F) {
    let t101342 = t2173 * t15573 * t28957;
    let t101355 = t15573 * t28951;
    let t101356 = t2173 * t101355;
    let t101363 = -0.33163888888888888888e-2 * t100575 - 0.13901041666666666667e-2 * t7703 * t27772 * t100975 * t1003 + 0.16581944444444444444e-2 * t100578 - t96019 - 0.13901041666666666667e-2 * t7687 * t28952 + 0.37069444444444444445e-2 * t7696 * t28952 - 0.46336805555555555557e-3 * t101356 - 0.16581944444444444444e-2 * t100580 - 0.44218518518518518516e-2 * t100583 + 0.3684876543209876543e-2 * t100586 + 0.69505208333333333333e-3 * t7687 * t28948;
    (t101342, t101355, t101363)
}
