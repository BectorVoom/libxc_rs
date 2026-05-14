//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 909/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk909<F: Float>(t538: F, t554: F, t58: F, t22591: F, t129: F, t22708: F, t135: F, t22711: F, t138: F, t1995: F, t23809: F, t1354: F, t527: F) -> (F, F, F, F, F) {
    let t23849 = t58 * t538 * t554;
    let t23850 = t22591 * t23849;
    let t23855 = t129 * t22708;
    let t23856 = t22711 * t135;
    let t23857 = t23856 * t138;
    let t23866 = t1995 * t23809;
    let t23869 = t527 * t1354;
    (t23850, t23855, t23857, t23866, t23869)
}
