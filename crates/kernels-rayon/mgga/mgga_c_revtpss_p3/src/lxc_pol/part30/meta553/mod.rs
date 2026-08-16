//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta553(t4537: f64, t775: f64, t1544: f64, t2832: f64, t890: f64, t2411: f64, t14365: f64, t1214: f64, t5428: f64, t1298: f64, t5501: f64, t1448: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t61182, t61203, t63164, t63186, t72861, t73262, t73394) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1992(t4537, t775, t1544, t2832, t890, t2411, t14365, t1214, t5428, t1298, t5501, t1448, t5591);
    (t61182, t61203, t63164, t63186, t72861, t73262, t73394)
}
