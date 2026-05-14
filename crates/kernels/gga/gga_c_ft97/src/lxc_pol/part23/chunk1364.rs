//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1364/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1364<F: Float>(t123784: F, t1472: F, t19044: F, t25057: F, t218: F, t5284: F, t811: F, t820: F, t127170: F, t28660: F, t24330: F, t25049: F, t31420: F, t27574: F, t31410: F, t28652: F) -> (F, F, F, F, F, F, F, F) {
    let t127299 = t1472 * t123784;
    let t127301 = t25057 * t19044;
    let t127304 = t218 * t5284;
    let t127306 = t25057 * t127304 * t811;
    let t127310 = t25057 * t127304 * t820;
    let t127319 = t28660 * t127170;
    let t127322 = t25049 * t24330 * t31420;
    let t127324 = t27574 * t31410;
    let t127325 = t28652 * t127324;
    (t127299, t127301, t127306, t127310, t127319, t127322, t127324, t127325)
}
