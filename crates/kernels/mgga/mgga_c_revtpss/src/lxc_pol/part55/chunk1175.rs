//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1175/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1175<F: Float>(t32192: F, t5629: F, t8583: F, t8589: F, t121354: F, t33969: F, t8591: F, t120991: F, t121019: F, t5676: F, t121018: F, t5674: F, t94396: F) -> (F, F, F, F) {
    let t125721 = t8583 * t8589 * t32192 * t5629;
    let t125732 = t8591 * t121354 * t33969;
    let t125749 = t120991 * t121019 * t5676;
    let t125753 = t121018 * t121019 * t5674 * t94396;
    (t125721, t125732, t125749, t125753)
}
