//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 492/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk492<F: Float>(t5968: F, t605: F, t144: F, t1901: F, t28: F, t446: F, t5854: F, t5857: F, t5862: F, t5866: F, t5871: F, t5875: F, t5880: F, t5882: F, t5886: F, t5931: F, t5937: F, t5941: F, t5944: F, t5949: F, t5953: F, t5958: F, t89: F) -> (F, F, F) {
    let t5969 = t605 * t5968;
    let t5970 = t144 * t5969;
    let t5973 = t5854 + t1901 * t5857 / 9.0 + 2.0 / 3.0 * t446 * t5862 - t446 * t5866 / 3.0 + t446 * t5871 / 3.0 - t446 * t5875 / 3.0 - t5880 - t446 * t5882 / 9.0 - t446 * t5886 / 3.0 + t89 * t28 * t5931 / 3.0 - t446 * t5937 / 3.0 + t5941 + t1901 * t5944 / 9.0 + t446 * t5949 / 3.0 - t446 * t5953 / 3.0 + 2.0 / 3.0 * t446 * t5958 - t446 * t5970 / 3.0;
    (t5969, t5970, t5973)
}
