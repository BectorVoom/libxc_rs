//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta851(t3271: f64, t43889: f64, t5992: f64, t11243: f64, t5999: f64, t43880: f64, t11265: f64, t63323: f64, t63327: f64, t63330: f64, t63848: f64, t63853: f64, t63856: f64, t63858: f64, t63860: f64, t63862: f64, t63865: f64, t63867: f64, t18520: f64, t699: f64, t2403: f64, t6011: f64, t136: f64, t3297: f64, t63357: f64, t6014: f64, t1113: f64, t63363: f64, t44938: f64, t48140: f64, t55716: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63870, t63873, t63876, t63879, t63881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079(t3271, t43889, t5992, t11243, t5999, t43880, t11265, t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867);
        let (t63886, t63888, t63891, t63893, t63896, t63899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3080(t18520, t699, t2403, t6011, t136, t3297, t63357, t6014, t1113, t63363, t44938, t48140, t55716);
    (t63870, t63873, t63876, t63879, t63881, t63886, t63888, t63891, t63893, t63896, t63899)
}
