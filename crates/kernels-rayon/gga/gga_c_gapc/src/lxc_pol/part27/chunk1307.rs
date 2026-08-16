//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1307/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1307(t224: f64, t33137: f64, t34298: f64, t35717: f64, t36081: f64, t11283: f64, t11297: f64, t12006: f64, t987: f64, t33103: f64, t33105: f64, t33106: f64, t33110: f64, t33113: f64, t33114: f64, t33116: f64, t33119: f64, t33144: f64, t33147: f64, t34285: f64, t34287: f64, t34295: f64) -> (f64, f64, f64) {
    let t36084 = t224 * (t33137 + t34298 + t35717 + t36081);
    let t36089 = 4.0_f64 * t11283;
    let t36090 = 2.0_f64 * t11297;
    let t38845 = t987 * t12006;
    let t38846 = t33103 - t33105 + t33106 - t33110 - t33113 - t33114 + t33116 - t33119 + t36084 - t33144 + t33147 - t34285 + t34287 + t34295 + t38845;
    (t36089, t36090, t38846)
}
