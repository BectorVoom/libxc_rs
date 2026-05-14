//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 933/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk933<F: Float>(t28533: F, t505: F, t2665: F, t446: F, t28491: F, t28494: F, t28499: F, t28504: F, t28509: F, t28514: F, t28518: F, t28522: F, t28526: F, t28529: F, t28531: F, t1701: F, t4125: F, t6027: F) -> (F, F, F, F) {
    let t28534 = t28533 * t505;
    let t28535 = t2665 * t28534;
    let t28536 = t446 * t28535;
    let t28538 = t28491 / 9.0 - t28494 / 36.0 + t28499 / 3.0 + t28504 / 3.0 + t28509 / 3.0 + t28514 / 12.0 - 2.0 / 9.0 * t28518 - 2.0 / 9.0 * t28522 + 2.0 / 27.0 * t28526 + t28529 / 18.0 - t28531 / 27.0 + t28536 / 9.0;
    let t28540 = t1701 * t6027 * t4125;
    (t28535, t28536, t28538, t28540)
}
