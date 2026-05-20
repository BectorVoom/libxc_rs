//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1684/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1684<F: Float>(t19680: F, t70: F, t18281: F, t36: F, t5826: F, t627: F, t1486: F, t4181: F, t4187: F, t1470: F, t4217: F, t1494: F, t21686: F, t21687: F, t21690: F, t4182: F, t5820: F, t5827: F, t5830: F, t641: F, t85: F) -> (F, F) {
    let t21695 = t19680 * t70;
    let t21698 = t36 * t18281;
    let t21699 = t21698 * t70;
    let t21702 = t5826 * t627;
    let t21707 = t4181 * t1486;
    let t21710 = t4187 * t1486;
    let t21713 = t1470 * t4217;
    let t21720 = -t21686 * t21687 / F::new(6.0) - t21690 * t85 / F::new(12.0) - t5820 * t641 / F::new(12.0) - t21695 * t85 / F::new(12.0) - t21699 * t85 / F::new(12.0) - t21702 * t85 / F::new(12.0) - t5827 * t641 / F::new(12.0) - t21707 * t85 / F::new(6.0) - t21710 * t85 / F::new(6.0) - t21713 * t85 / F::new(6.0) - t5830 * t641 / F::new(6.0) - t4182 * t1494 / F::new(6.0);
    (t21698, t21720)
}
