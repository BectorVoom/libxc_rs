//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1304/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1304(t224: f64, t33137: f64, t34298: f64, t35717: f64, t36081: f64, t2036: f64, t33103: f64, t33105: f64, t33106: f64, t33110: f64, t33113: f64, t33114: f64, t33116: f64, t33119: f64, t33144: f64, t33147: f64, t34285: f64, t34287: f64, t3797: f64) -> f64 {
    let t36084 = t224 * (t33137 + t34298 + t35717 + t36081);
    let t36085 = t2036 * t3797 + t33103 - t33105 + 2.0_f64 * t33106 - t33110 - t33113 - t33114 + t33116 - t33119 - t33144 + t33147 - t34285 + t34287 + t36084;
    t36085
}
