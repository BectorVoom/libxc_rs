//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1039/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1039<F: Float>(t7374: F, t8634: F, t649: F, t8686: F, t1937: F, t26399: F, t28658: F, t6993: F, t7359: F, t3140: F, t860: F, t8477: F) -> (F, F, F, F, F, F, F) {
    let t32404 = t8634 * t7374;
    let t32415 = t649 * t8686;
    let t32417 = F::new(2.0) * t26399 * t1937;
    let t32419 = F::new(2.0) * t28658 * t1937;
    let t32421 = F::new(2.0) * t7359 * t6993;
    let t32425 = t860 * t3140;
    let t32426 = t8477 * t32425;
    (t32404, t32415, t32417, t32419, t32421, t32425, t32426)
}
