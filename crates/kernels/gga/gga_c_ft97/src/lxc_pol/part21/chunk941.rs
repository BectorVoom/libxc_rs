//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 941/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk941<F: Float>(t29721: F, t8270: F, t1317: F, t28: F, t23124: F, t26022: F, t26025: F, t26036: F, t29672: F, t29676: F, t29680: F, t29684: F, t29690: F, t29695: F, t29699: F, t29704: F, t29709: F, t29714: F, t29719: F) -> (F, F, F) {
    let t29722 = t8270 * t29721;
    let t29724 = t1317 * t28 * t29722;
    let t29725 = -t29672 / 3.0 - t29676 / 3.0 + 2.0 / 3.0 * t29680 + 4.0 / 3.0 * t29684 - 2.0 / 9.0 * t26022 - t26025 / 18.0 - 2.0 * t29690 - t29695 / 8.0 + t26036 / 9.0 - t23124 - 4.0 / 9.0 * t29699 + t29704 / 3.0 + 2.0 / 3.0 * t29709 + t29714 / 12.0 + t29719 / 6.0 - t29724;
    (t29722, t29724, t29725)
}
