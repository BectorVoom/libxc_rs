//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1989/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989<F: Float>(t4147: F, t7311: F, t1925: F, t36: F, t606: F, t7933: F, t1450: F, t11239: F, t3268: F, t211: F, t9644: F, t138: F, t785: F, t9302: F) -> (F, F, F, F, F, F, F) {
    let t32113 = t4147 * t7311;
    let t32591 = t1925 * t36;
    let t32592 = t32591 * t606;
    let t33651 = t4147 * t7933;
    let t35070 = t7311 * t1450;
    let t36870 = t11239 * t3268;
    let t39643 = F::cast_from(1.0_f64) / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    (t32113, t32592, t33651, t35070, t36870, t39643, t40270)
}
