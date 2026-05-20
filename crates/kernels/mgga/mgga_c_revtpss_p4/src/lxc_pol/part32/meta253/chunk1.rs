//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1060/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1060<F: Float>(t6929: F, t6933: F, t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5884: F, t5887: F, t5921: F, t651: F, t6765: F, t6773: F) -> (F, F) {
    let t6934 = t6929 + t6933;
    let t6936 = -t118 * t6765 - F::new(2.0) * t1502 * t1843 - F::new(4.0) * t1519 * t4248 + F::new(2.0) * t1847 * t1911 - t508 * t5877 - F::new(2.0) * t508 * t5884 + t511 * t6934 + t569 * t6773 - F::new(4.0) * t5887 * t651 - F::new(2.0) * t5921 * t651;
    (t6934, t6936)
}
