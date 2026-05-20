//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1828/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1828<F: Float>(t1204: F, t3766: F, t3153: F, t3588: F, t5480: F, t3555: F, t3754: F) -> (F, F, F, F) {
    let t12702 = t1204 * t3766;
    let t12705 = t3588 * t3153;
    let t12706 = t12705 * t5480;
    let t12709 = t3555 * t3754;
    (t12702, t12705, t12706, t12709)
}
