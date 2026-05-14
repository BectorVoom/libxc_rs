//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1342/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1342<F: Float>(t114628: F, t114634: F, t114636: F, t114638: F, t118707: F, t118766: F, t119560: F, t119563: F, t119566: F, t119569: F, t119573: F, t119577: F, t119580: F, t32008: F, t32066: F, t34697: F) -> (F,) {
    let t119586 = 0.88437037037037037035e-2 * t114628 + t114634 - 0.24872916666666666666e-2 * t119560 + 0.16581944444444444444e-2 * t119563 + 0.13265555555555555555e-1 * t119566 + 0.24872916666666666666e-2 * t119569 - 0.26805555555555555557e-2 * t32008 * t118707 + t114636 + 0.11054629629629629629e-2 * t119573 - t114638 - 0.24872916666666666666e-2 * t119577 + 0.49745833333333333332e-2 * t119580 + 0.40208333333333333335e-2 * t32066 * t34697 + 0.17870370370370370371e-2 * t32008 * t118766;
    (t119586,)
}
