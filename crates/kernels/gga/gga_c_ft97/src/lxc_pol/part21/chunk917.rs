//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 917/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk917<F: Float>(t23890: F, t23914: F, t23920: F, t24034: F, t24041: F, t27116: F, t27121: F, t27126: F, t27130: F, t27133: F, t27135: F, t27139: F, t23931: F, t27145: F, t27150: F, t27155: F, t27161: F, t27163: F, t27168: F, t27171: F, t27176: F, t27179: F, t27183: F, t27187: F) -> (F, F) {
    let t27376 = -t27116 / 3.0 + t23890 / 18.0 - t24034 - t23914 / 27.0 + t23920 / 9.0 - t27121 / 9.0 + t27126 / 12.0 + t27130 / 3.0 + t27133 / 3.0 - t27135 / 36.0 - t24041 + t27139 / 18.0;
    let t27389 = t27145 / 9.0 - t27150 / 6.0 - t27155 / 6.0 - t27161 / 8.0 - t27163 / 54.0 + t27168 / 18.0 + t27171 / 9.0 - 2.0 / 9.0 * t23931 + 2.0 / 3.0 * t27176 - 2.0 / 9.0 * t27179 + 2.0 / 3.0 * t27183 + 2.0 / 3.0 * t27187;
    (t27376, t27389)
}
