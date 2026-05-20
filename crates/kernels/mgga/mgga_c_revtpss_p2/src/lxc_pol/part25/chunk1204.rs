//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1204/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1204<F: Float>(t1936: F, t670: F, t4147: F, t7311: F, t11239: F, t3268: F, t2645: F, t4366: F, t837: F, t211: F, t9644: F, t138: F, t785: F, t9302: F) -> (F, F, F, F, F, F, F) {
    let t28264 = t670 * t1936;
    let t32113 = t4147 * t7311;
    let t36870 = t11239 * t3268;
    let t39588 = t4366 * t2645;
    let t39620 = t837 * t2645;
    let t39643 = F::new(1.0) / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    (t28264, t32113, t36870, t39588, t39620, t39643, t40270)
}
