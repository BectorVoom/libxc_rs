//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 695/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk695(t24886: f64, t4266: f64, t1495: f64, t2766: f64, t4141: f64, t11593: f64, t1901: f64, t24882: f64, t24884: f64, t29052: f64, t29057: f64, t29060: f64, t29064: f64, t29068: f64, t29073: f64, t29077: f64, t29084: f64, t29087: f64) -> f64 {
    let t29090 = t24886 * t4266;
    let t29093 = t2766 * t1495;
    let t29094 = t29093 * t4141;
    let t29097 = -2.0_f64 / 3.0_f64 * t1901 * t29052 - 2.0_f64 / 3.0_f64 * t1901 * t29057 + t1901 * t29060 / 9.0_f64 + t1901 * t29064 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t11593 * t29068 - 2.0_f64 * t1901 * t29073 - 2.0_f64 / 3.0_f64 * t1901 * t29077 - t24882 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t24884 + t1901 * t29084 / 9.0_f64 + t1901 * t29087 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t29090 - 2.0_f64 / 27.0_f64 * t1901 * t29094;
    t29097
}
