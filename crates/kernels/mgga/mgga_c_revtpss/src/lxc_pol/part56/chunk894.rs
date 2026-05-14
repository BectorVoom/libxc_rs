//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 894/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk894<F: Float>(t1518: F, t8964: F, t1519: F, t1911: F, t33346: F, t33578: F, t33580: F, t33583: F, t33595: F, t33599: F, t33650: F, t33654: F, t33659: F, t34377: F, t34379: F, t34383: F, t34400: F, t34401: F, t34424: F, t34880: F, t569: F, t651: F, t7586: F, t8158: F, t8967: F) -> (F, F) {
    let t34882 = t8964 * t1518;
    let t34886 = -2.0 * t1519 * t33346 + t1911 * t8967 + t34880 * t569 - 2.0 * t34882 * t651 - 4.0 * t7586 * t8158 - t33578 - t33580 - t33583 - t33595 - t33599 - t33650 - t33654 + t33659 - 4.0 * t34377 - 4.0 * t34379 - 4.0 * t34383 + 2.0 * t34400 + 2.0 * t34401 - 2.0 * t34424;
    (t34882, t34886)
}
