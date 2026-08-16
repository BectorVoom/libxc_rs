//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2071;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta488(t1614: f64, t2967: f64, t1626: f64, t2986: f64, t4587: f64, t914: f64, t936: f64, t2919: f64, t4590: f64, t1596: f64, t2923: f64, t2927: f64, t11289: f64, t1610: f64, t2869: f64, t4632: f64, t15125: f64, t15168: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15163: f64, t15166: f64, t15170: f64, t15173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15406, t15413, t15416, t15418, t15420, t15421, t15423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2071(t1614, t2967, t1626, t2986, t4587, t914, t936, t2919, t4590, t1596, t2923, t2927);
        let (t15425, t15427, t15435, t15450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2072(t11289, t1610, t2869, t4632, t15125, t15168, t15137, t15142, t15147, t15151, t15156, t15160, t15163, t15166, t15170, t15173);
    (t15406, t15413, t15416, t15418, t15420, t15421, t15423, t15425, t15427, t15435, t15450)
}
