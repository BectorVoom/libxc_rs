//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 846/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk846<F: Float>(t23026: F, t23029: F, t23034: F, t23038: F, t23041: F, t23045: F, t23048: F, t23053: F, t23055: F, t23060: F, t23064: F, t23068: F, t23072: F, t23076: F, t23079: F, t23081: F) -> (F,) {
    let t23083 = -t23026 / 2.0 + t23029 / 3.0 - 2.0 / 3.0 * t23034 - t23038 + 4.0 * t23041 + 2.0 * t23045 - 4.0 / 3.0 * t23048 - t23053 - t23055 / 9.0 + t23060 / 3.0 + t23064 / 6.0 + t23068 / 9.0 - t23072 / 3.0 - t23076 - t23079 + 2.0 / 3.0 * t23081;
    (t23083,)
}
