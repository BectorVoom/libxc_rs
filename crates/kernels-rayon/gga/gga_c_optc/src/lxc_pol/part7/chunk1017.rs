//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1017/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1017(t1928: f64, t6785: f64, t6931: f64, t2030: f64, t6782: f64, t1948: f64, t6936: f64, t2034: f64, t2080: f64, t2093: f64, t654: f64, t6904: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22218 = t6785 * t1928;
    let t22219 = t6931 * t22218;
    let t22222 = t2030 * t6782;
    let t22224 = t6936 * t1948;
    let t22225 = t2034 * t22224;
    let t22228 = t2080 * t2093;
    let t22230 = t654 * t6904;
    (t22218, t22219, t22222, t22224, t22225, t22228, t22230)
}
