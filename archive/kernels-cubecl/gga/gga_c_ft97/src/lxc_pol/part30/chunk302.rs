//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 302/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk302<F: Float>(t1701: F, t3780: F, t811: F, t1200: F, t1471: F, t820: F, t4093: F, t800: F) -> (F, F, F, F) {
    let t4096 = t1701 * t3780 * t811;
    let t4099 = t1200 * t1471;
    let t4100 = t3780 * t820;
    let t4101 = t1701 * t4100;
    let t4104 = t800 * t4093;
    (t4096, t4099, t4101, t4104)
}
