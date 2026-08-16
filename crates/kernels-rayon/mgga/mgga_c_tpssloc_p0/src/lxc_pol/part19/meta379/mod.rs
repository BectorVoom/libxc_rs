//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta379(t43819: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64, t43936: f64, t449: f64, t300: f64, t1098: f64, t11470: f64, t1119: f64, t11180: f64, t3308: f64, t3256: f64, t3312: f64, t3316: f64, t11270: f64, t3259: f64, t1094: f64, t11274: f64, t11278: f64, t3262: f64, t3311: f64, t409: f64, t3265: f64, t11277: f64, t11634: f64, t3411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43949 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416(t43819, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828);
        let (t43951, t43953, t43956, t43958, t43961) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417(t43936, t43949, t449, t300, t1098, t11470, t1119, t11180, t3308, t3256, t3312, t3316);
        let (t43963, t43966, t43970, t43973, t43975) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418(t11270, t3259, t1094, t11274, t11278, t3262, t3311, t409, t3265, t11277, t11634, t3411);
    (t43951, t43953, t43956, t43958, t43961, t43963, t43966, t43970, t43973, t43975)
}
