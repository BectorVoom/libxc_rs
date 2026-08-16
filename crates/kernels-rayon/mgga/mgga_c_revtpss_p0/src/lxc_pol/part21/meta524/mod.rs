//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2163;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta524(t1082: f64, t15648: f64, t3291: f64, t4757: f64, t3059: f64, t5004: f64, t16426: f64, t3318: f64, t1043: f64, t1089: f64, t4930: f64, t15717: f64, t3286: f64, t4746: f64, t1071: f64, t3316: f64, t342: f64, t1647: f64, t3298: f64, t16183: f64, t378: f64, t4980: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16479, t16482, t16485, t16488, t16496, t16499) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2163(t1082, t15648, t3291, t4757, t3059, t5004, t16426, t3318, t1043, t1089, t4930, t15717);
        let (t16502, t16505, t16506, t16509, t16515, t16520) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2164(t3286, t4746, t1071, t3316, t342, t1647, t3298, t1089, t16183, t378, t4980, t989);
    (t16479, t16482, t16485, t16488, t16496, t16499, t16502, t16505, t16506, t16509, t16515, t16520)
}
