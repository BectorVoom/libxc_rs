//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 696/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk696(t24890: f64, t4146: f64, t28983: f64, t296: f64, t29020: f64, t24886: f64, t4151: f64, t7102: f64, t8392: f64, t15191: f64, t6274: f64, t29045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29098 = t24890 * t4146;
    let t29101 = t296 * t28983;
    let t29104 = t296 * t29020;
    let t29107 = t24886 * t4151;
    let t29111 = t8392 * t7102;
    let t29113 = t15191 * t6274;
    let t29116 = t296 * t29045;
    (t29098, t29101, t29104, t29107, t29111, t29113, t29116)
}
