//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 1000/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1000(t1882: f64, t4178: f64, t4183: f64, t1255: f64, t2413: f64, t835: f64, t2405: f64, t2857: f64, t10447: f64, t4151: f64, t14116: f64, t4140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15500 = 2.0_f64 / 9.0_f64 * t1882 * t4178;
    let t15502 = 4.0_f64 / 9.0_f64 * t1882 * t4183;
    let t15504 = t835 * t1255 * t2413;
    let t15508 = t2857 * t1255 * t2405;
    let t15511 = t10447 * t4151;
    let t15514 = t4140 * t14116;
    (t15500, t15502, t15504, t15508, t15511, t15514)
}
