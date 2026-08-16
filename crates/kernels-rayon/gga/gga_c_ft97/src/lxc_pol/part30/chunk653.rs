//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 653/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk653(t2781: f64, t28501: f64, t1486: f64, t193: f64, t1476: f64, t4129: f64, t7021: f64, t856: f64, t852: f64, t6308: f64, t4255: f64, t6334: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28502 = t2781 * t28501;
    let t28504 = t1486 * t193 * t28502;
    let t28506 = t1476 * t4129;
    let t28507 = t2781 * t28506;
    let t28509 = t1486 * t193 * t28507;
    let t28511 = t7021 * t856;
    let t28512 = t852 * t28511;
    let t28514 = t6308 * t193 * t28512;
    let t28516 = t6334 * t4255;
    (t28504, t28506, t28509, t28511, t28514, t28516)
}
