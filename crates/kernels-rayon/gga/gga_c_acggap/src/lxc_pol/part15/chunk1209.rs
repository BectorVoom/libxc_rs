//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1209/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1209(t30249: f64, t32397: f64, t32398: f64, t32401: f64, t32403: f64, t32404: f64, t33960: f64, t33968: f64, t33970: f64, t33984: f64, t36877: f64, t36888: f64, t36890: f64, t36892: f64, t38890: f64, t38894: f64, t38899: f64, t38903: f64) -> f64 {
    let t41390 = 0.4584375e-1_f64 * t38890 + 0.305625e-1_f64 * t38894 - 0.1528125e-1_f64 * t33960 + t36877 + t33968 + t32397 + t32398 + t32401 + t33970 + t32403 - t32404 - 0.90702367218671976884e-1_f64 * t30249 + 0.85748036236139473944e-3_f64 * t38899 - t36888 + 0.75475421495049964965e-2_f64 * t33984 + t36890 + 0.37737710747524982483e-2_f64 * t38903 + t36892;
    t41390
}
