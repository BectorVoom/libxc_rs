//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta751 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2623;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta751(t1831: f64, t40292: f64, t12345: f64, t5314: f64, t12211: f64, t16296: f64, t40018: f64, t5223: f64, t16379: f64, t40021: f64, t12282: f64, t5234: f64, t3809: f64, t120: f64, t16205: f64, t12283: f64, t16227: f64, t1351: f64, t5286: f64, t12189: f64, t5227: f64, t16232: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53917, t53919, t53921, t53927, t53929, t53945) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2623(t1831, t40292, t12345, t5314, t12211, t16296, t40018, t5223, t16379, t40021, t12282, t5234);
        let (t53946, t53958, t53965, t53973, t53984, t53990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2624(t3809, t53945, t120, t16205, t12283, t16227, t1351, t5286, t12189, t5227, t16232, t3777);
    (t53917, t53919, t53921, t53927, t53929, t53945, t53946, t53958, t53965, t53973, t53984, t53990)
}
