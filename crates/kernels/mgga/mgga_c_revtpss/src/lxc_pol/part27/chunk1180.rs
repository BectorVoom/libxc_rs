//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1180/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1180<F: Float>(t11239: F, t3736: F, t2645: F, t4366: F, t837: F, t211: F, t9644: F, t138: F, t785: F, t9302: F, t10818: F, t221: F) -> (F, F, F, F, F, F) {
    let t37885 = t11239 * t3736;
    let t39588 = t4366 * t2645;
    let t39620 = t837 * t2645;
    let t39643 = F::new(1.0) / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40419 = t221 * t10818;
    (t37885, t39588, t39620, t39643, t40270, t40419)
}
