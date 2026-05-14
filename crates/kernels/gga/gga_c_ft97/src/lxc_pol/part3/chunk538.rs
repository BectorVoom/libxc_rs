//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 538/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk538<F: Float>(t1901: F, t2164: F, t28: F, t3460: F, t3489: F, t3545: F, t3551: F, t446: F, t4726: F, t4730: F, t4735: F, t4739: F, t4743: F, t4747: F, t4792: F, t4807: F, t4811: F, t4815: F, t4819: F, t4824: F, t4829: F, t4833: F, t89: F) -> (F,) {
    let t4837 = 2.0 / 3.0 * t446 * t4726 + 2.0 / 3.0 * t446 * t4730 + 2.0 / 3.0 * t446 * t4735 - 2.0 / 9.0 * t446 * t4739 - t446 * t4743 / 9.0 - 2.0 / 27.0 * t446 * t4747 + t2164 - 2.0 / 9.0 * t3489 + 2.0 / 9.0 * t3551 + 2.0 / 9.0 * t3545 + t89 * t28 * t4792 / 3.0 - t446 * t4807 / 3.0 - 2.0 / 3.0 * t446 * t4811 - 2.0 / 3.0 * t446 * t4815 - t446 * t4819 / 3.0 + 2.0 / 9.0 * t1901 * t4824 + 2.0 / 9.0 * t1901 * t4829 + 2.0 / 9.0 * t446 * t4833 + 2.0 / 27.0 * t3460;
    (t4837,)
}
