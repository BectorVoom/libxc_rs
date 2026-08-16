//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1974;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta436(t15292: f64, t15330: f64, t15386: f64, t15423: f64, t225: f64, t3507: f64, t475: f64, t6739: f64, t1755: f64, t11546: f64, t14726: f64, t15026: f64, t3032: f64, t3514: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15425, t15426, t15429, t15430, t15434, t15437) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1974(t15292, t15330, t15386, t15423, t225, t3507, t475, t6739, t1755, t11546, t14726, t15026, t3032);
        let t15438 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1975(t15437, t3514);
    (t15425, t15426, t15429, t15430, t15434, t15437, t15438)
}
