//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 828/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk828<F: Float>(t1904: F, t22908: F, t22907: F, t1286: F, t22495: F, t22500: F, t22504: F, t22507: F, t22509: F, t22865: F, t22868: F, t22872: F, t22875: F, t22879: F, t22886: F, t22893: F, t22896: F, t22900: F, t22904: F, t5495: F, t5501: F, t5510: F, t5620: F, t5624: F) -> (F, F, F) {
    let t22909 = t22908 * t1904;
    let t22910 = t22907 * t22909;
    let t22913 = -2.0 / 3.0 * t1286 * t22495 - t1286 * t22500 / 3.0 + 8.0 * t22504 + 4.0 * t22507 + 8.0 * t22509 + t1286 * t22865 / 6.0 - t22868 / 9.0 + t22872 - 2.0 / 3.0 * t1286 * t22875 + 2.0 / 9.0 * t22879 - 2.0 / 3.0 * t5495 * t5510 + t1286 * t22886 + t5495 * t5624 / 3.0 + t5495 * t5620 / 3.0 - t22893 / 9.0 + t1286 * t22896 / 3.0 + t1286 * t22900 / 6.0 + t5501 * t22904 / 9.0 + 2.0 / 9.0 * t5501 * t22910;
    (t22909, t22910, t22913)
}
