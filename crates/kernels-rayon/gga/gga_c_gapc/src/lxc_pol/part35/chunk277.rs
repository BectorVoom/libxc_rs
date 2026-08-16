//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 277/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk277(t1043: f64, t1046: f64, t1014: f64, t1020: f64, t1024: f64, t1028: f64, t1041: f64) -> f64 {
    let t1047 = t1043 * t1046;
    let t1049 = 0.13900948042322754167e-2_f64 * t1014 + 0.10120768229166666667e-4_f64 * t1020 - 0.86880925264517213544e-4_f64 * t1024 - 0.11594181388521408695e-4_f64 * t1028 - 0.84412963981222021454e-7_f64 * t1041 + 0.72463633678258804342e-6_f64 * t1047;
    t1049
}
