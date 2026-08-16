//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2035;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta464(t221: f64, t3734: f64, t5196: f64, t3726: f64, t5206: f64, t12199: f64, t5202: f64, t118: f64, t5187: f64, t794: f64, t3739: f64, t16018: f64, t210: f64, t214: f64, t12225: f64, t16095: f64, t2586: f64, t12236: f64, t1315: f64, t16083: f64, t16086: f64, t16090: f64, t16099: f64, t16101: f64, t5195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16103, t16106, t16108, t16111, t16113, t16115) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2035(t221, t3734, t5196, t3726, t5206, t12199, t5202, t118, t5187, t794, t3739, t16018, t210, t214);
        let (t16118, t16119, t16121) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2036(t12225, t16095, t2586, t12236, t1315, t16083, t16086, t16090, t16099, t16101, t16103, t16106, t16108, t16113, t16115, t5195);
    (t16103, t16106, t16108, t16111, t16113, t16115, t16118, t16119, t16121)
}
