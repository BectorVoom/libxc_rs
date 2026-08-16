//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2921/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2921<F: Float>(t10175: F, t9686: F, t1420: F, t4075: F, t786: F, t2439: F, t3895: F, t4132: F, t1359: F, t39501: F, t10115: F, t555: F) -> (F, F, F, F, F) {
    let t47527 = t10175 * t9686;
    let t47530 = t786 * t1420 * t4075;
    let t47534 = t2439 * t3895 * t4132;
    let t47561 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t1359;
    let t47567 = t10115 * t555;
    (t47527, t47530, t47534, t47561, t47567)
}
