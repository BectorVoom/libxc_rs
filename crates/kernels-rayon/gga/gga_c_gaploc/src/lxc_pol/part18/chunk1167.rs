//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1167/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1167(t10232: f64, t29860: f64, t29862: f64, t29865: f64, t29868: f64, t29871: f64, t29876: f64, t29879: f64, t29892: f64, t31565: f64, t31568: f64, t31570: f64, t31575: f64, t31577: f64, t4141: f64) -> f64 {
    let t31578 = t29860 - t29862 - t29865 - t29868 + t29871 + t29876 - t29879 - t31565 - t31568 + t29892 + t31570 - 0.31616674039640166222e-2_f64 * t4141 * t10232 + t31575 + t31577;
    t31578
}
