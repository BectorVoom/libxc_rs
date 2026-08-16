//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3681/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3681<F: Float>(t1196: F, t20895: F, t3498: F, t16673: F, t5192: F, t69571: F, t69573: F, t69575: F, t69577: F, t69579: F, t69581: F, t69583: F, t69585: F, t69587: F, t69590: F, t69594: F) -> (F, F, F) {
    let t69603 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t20895 * t3498;
    let t69605 = F::cast_from(0.69263436422725855034e2_f64) * t5192 * t16673;
    let t69606 = -t69571 + t69573 - t69575 + t69577 + t69579 - t69581 - t69583 - t69585 + t69587 - t69590 + t69594 - t69603 - t69605;
    (t69603, t69605, t69606)
}
