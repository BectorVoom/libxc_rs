//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 782/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk782<F: Float>(t2826: F, t2654: F, t2657: F, t2839: F, t2668: F, t2694: F, t123: F, t721: F, t776: F, t780: F, t39: F, t55: F, t59: F, t87: F, t2693: F, t754: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11522 = 4.0 * t2826;
    let t11525 = 0.1929837539843104208e3 * t2654;
    let t11526 = 24.0 * t2657;
    let t11529 = 0.4155806185363551302e3 * t2839;
    let t11534 = 24.0 * t2668;
    let t11536 = 4.0 * t2694;
    let t11545 = 0.22911460125803964958e1 * t721 * t123 * t776 * t780;
    let t11549 = 24.0 * t39 * t55 * t59 * t87;
    let t11552 = 0.71233333333333333332e-1 * t721 * t754 * t2693;
    (t11522, t11525, t11526, t11529, t11534, t11536, t11545, t11549, t11552)
}
