//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1311/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1311<F: Float>(t105414: F, t105971: F, t119598: F, t119602: F, t119606: F, t119610: F, t119614: F, t119618: F, t119623: F, t119627: F, t119631: F, t119635: F, t105973: F, t105981: F, t119642: F, t119644: F, t119649: F, t119652: F, t119655: F, t119659: F, t119664: F, t119668: F, t119672: F, t119677: F) -> (F, F) {
    let t120903 = -t119598 / 9.0 + t119602 / 24.0 + t119606 / 2.0 - t119610 / 9.0 - t119614 / 9.0 + t119618 / 27.0 - t119623 / 36.0 - 2.0 / 27.0 * t119627 - t119631 / 18.0 + 4.0 * t119635 - t105971 - 2.0 / 27.0 * t105414;
    let t120915 = -t119642 / 36.0 + 2.0 / 27.0 * t119644 + t105973 + 2.0 / 3.0 * t119649 - t119652 / 9.0 - 2.0 / 9.0 * t119655 + t105981 + 2.0 / 27.0 * t119659 + t119664 / 3.0 + t119668 / 3.0 + t119672 / 12.0 + 2.0 / 3.0 * t119677;
    (t120903, t120915)
}
