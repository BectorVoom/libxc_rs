//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1220/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1220<F: Float>(t101592: F, t101601: F, t101606: F, t101609: F, t101613: F, t101619: F, t101623: F, t102173: F, t102175: F, t102176: F, t102181: F, t101638: F, t101661: F, t101628: F, t101631: F, t101636: F, t101642: F, t101646: F, t101650: F, t101655: F, t101659: F, t101665: F, t101669: F) -> (F, F) {
    let t102184 = -t102173 - 3.0 / 8.0 * t101592 - t102175 + t102176 - 8.0 / 3.0 * t101601 + 3.0 / 2.0 * t101606 - t101609 / 3.0 + 2.0 * t101613 + t102181 + t101619 / 6.0 - 2.0 / 3.0 * t101623;
    let t102188 = 4.0 / 9.0 * t101638;
    let t102193 = t101661 / 3.0;
    let t102196 = 2.0 / 9.0 * t101628 - 2.0 / 3.0 * t101631 + 3.0 * t101636 - t102188 - t101642 + t101646 / 6.0 - 3.0 * t101650 + 15.0 / 16.0 * t101655 - t101659 / 3.0 + t102193 + t101665 / 6.0 + 2.0 * t101669;
    (t102184, t102196)
}
