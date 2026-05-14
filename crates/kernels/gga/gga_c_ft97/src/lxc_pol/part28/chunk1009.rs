//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1009/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1009<F: Float>(t1369: F, t148678: F, t2112: F, t28: F, t139507: F, t139519: F, t139526: F, t139534: F, t148640: F, t148643: F, t148646: F, t148649: F, t148653: F, t148657: F, t148660: F, t148667: F, t148670: F, t148673: F, t148676: F) -> (F, F) {
    let t148681 = t1369 * t28 * t2112 * t148678;
    let t148683 = t148640 / 3.0 - 2.0 / 9.0 * t148643 + 2.0 / 9.0 * t148646 - 2.0 / 27.0 * t148649 + 2.0 / 9.0 * t148653 - 2.0 * t148657 + t148660 / 18.0 - t139507 / 27.0 + 2.0 / 27.0 * t139519 + t139526 / 18.0 - t139534 + t148667 / 3.0 + 4.0 / 9.0 * t148670 - 4.0 / 27.0 * t148673 - t148676 / 3.0 + t148681 / 3.0;
    (t148681, t148683)
}
