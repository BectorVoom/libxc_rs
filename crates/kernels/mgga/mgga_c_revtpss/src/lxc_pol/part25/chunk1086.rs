//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1086/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1086<F: Float>(t2022: F, t3999: F, t1931: F, t670: F, t197: F, t530: F, t2013: F, t8995: F, t2033: F, t9593: F, t1936: F, t4147: F, t7311: F, t11239: F, t3268: F, t2645: F, t4366: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27980 = t3999 * t2022;
    let t28025 = t1931 * t670;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    let t28264 = t670 * t1936;
    let t32113 = t4147 * t7311;
    let t36870 = t11239 * t3268;
    let t39588 = t4366 * t2645;
    (t27980, t28025, t28167, t28196, t28197, t28264, t32113, t36870, t39588)
}
