//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1263/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1263<F: Float>(t30232: F, t379: F, t5899: F, t95344: F, t119556: F, t119560: F, t119565: F, t119569: F, t119573: F, t119576: F, t119579: F, t119583: F, t119586: F, t119590: F, t30239: F, t23667: F) -> (F, F, F, F, F) {
    let t119592 = t30232 * t379;
    let t119594 = t5899 * t95344 * t119592;
    let t119595 = 3.0 * t119556 + 4.0 / 9.0 * t119560 - t119565 / 18.0 - 4.0 / 3.0 * t119569 - 2.0 / 3.0 * t119573 + t119576 + 2.0 / 3.0 * t119579 + t119583 - 4.0 / 3.0 * t119586 - t119590 / 6.0 + t119594;
    let t119596 = t30239 * t379;
    let t119598 = t5899 * t23667 * t119596;
    (t119592, t119594, t119595, t119596, t119598)
}
