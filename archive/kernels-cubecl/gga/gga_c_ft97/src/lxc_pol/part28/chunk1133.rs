//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1133/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1133<F: Float>(t2185: F, t23657: F, t27091: F, t5900: F, t32063: F, t34823: F, t7366: F, t28: F, t3526: F, t586: F, t5890: F, t7339: F) -> (F, F, F) {
    let t148353 = t23657 * t2185 * t5900 * t27091;
    let t148360 = t7366 * t32063 * t34823;
    let t148365 = t5890 * t28 * t586 * t7339 * t3526;
    (t148353, t148360, t148365)
}
