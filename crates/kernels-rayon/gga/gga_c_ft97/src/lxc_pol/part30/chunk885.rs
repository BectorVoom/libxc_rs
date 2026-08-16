//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 885/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk885(t1476: f64, t7124: f64, t840: f64, t871: f64, t2862: f64, t319: f64, t35863: f64, t1248: f64, t7611: f64, t1091: f64, t34202: f64, t2874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36112 = t1476 * t7124;
    let t36114 = t840 * t871 * t36112;
    let t36118 = t2862 * t319 * t35863;
    let t36121 = t7611 * t1248;
    let t36123 = t840 * t871 * t36121;
    let t36126 = t34202 * t1091;
    let t36127 = t2874 * t36126;
    (t36112, t36114, t36118, t36121, t36123, t36126, t36127)
}
