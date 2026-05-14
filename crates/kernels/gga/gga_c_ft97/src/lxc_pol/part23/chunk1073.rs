//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1073/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1073<F: Float>(t2681: F, t863: F, t4239: F, t848: F, t319: F, t43833: F, t10697: F, t43912: F, t10580: F, t871: F, t2680: F, t309: F, t2770: F, t43917: F, t799: F, t2766: F, t2843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t56127 = t2681 * t863;
    let t56180 = t848 * t4239;
    let t56339 = t43833 * t319;
    let t56352 = t848 * t10697;
    let t56418 = t43912 * t319;
    let t56437 = t10580 * t871;
    let t56456 = t2680 * t309;
    let t56522 = t2770 * t4239;
    let t56643 = t43917 * t319;
    let t56815 = t799 * t863;
    let t56819 = t2766 * t2843;
    (t56127, t56180, t56339, t56352, t56418, t56437, t56456, t56522, t56643, t56815, t56819)
}
