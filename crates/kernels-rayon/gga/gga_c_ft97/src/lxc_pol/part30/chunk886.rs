//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 886/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk886(t296: f64, t36066: f64, t1212: f64, t7672: f64, t2843: f64, t840: f64, t36064: f64, t1091: f64, t34207: f64, t2881: f64, t24890: f64, t7032: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36130 = t296 * t36066;
    let t36133 = t7672 * t1212;
    let t36135 = t840 * t2843 * t36133;
    let t36138 = t296 * t36064;
    let t36141 = t34207 * t1091;
    let t36142 = t2881 * t36141;
    let t36145 = t24890 * t7032;
    (t36130, t36133, t36135, t36138, t36141, t36142, t36145)
}
