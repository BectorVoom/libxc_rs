//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 566/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk566<F: Float>(t1175: F, t1179: F, t1178: F, t444: F, t439: F, t1187: F) -> (F, F, F, F) {
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    let t3495 = F::new(1.0) / t3494;
    let t3496 = t439 * t3495;
    let t3497 = t1187 * t1187;
    (t3491, t3495, t3496, t3497)
}
