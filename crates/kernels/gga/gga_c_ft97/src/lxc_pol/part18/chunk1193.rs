//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1193/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1193<F: Float>(t3188: F, t93409: F, t22953: F, t5674: F, t101588: F, t101592: F, t101596: F, t101598: F, t101601: F, t101606: F, t101609: F, t101613: F, t101616: F, t101619: F, t93434: F, t25928: F) -> (F, F, F, F, F) {
    let t101621 = t93409 * t3188;
    let t101623 = t5674 * t22953 * t101621;
    let t101625 = -t101588 - t101592 / 8.0 - t101596 + t101598 / 27.0 - 8.0 / 9.0 * t101601 + t101606 / 2.0 - t101609 / 9.0 + 2.0 / 3.0 * t101613 + t101616 + t101619 / 18.0 - 2.0 / 9.0 * t101623;
    let t101626 = t93434 * t3188;
    let t101628 = t5674 * t25928 * t101626;
    (t101621, t101623, t101625, t101626, t101628)
}
