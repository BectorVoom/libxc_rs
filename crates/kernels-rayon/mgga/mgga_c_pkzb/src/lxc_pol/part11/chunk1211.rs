//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1211/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1211(t10556: f64, t1535: f64, t1536: f64, t16825: f64, t16946: f64, t16950: f64, t20377: f64, t2718: f64, t29146: f64, t29149: f64, t29150: f64, t29151: f64, t6758: f64, t8758: f64, t8779: f64, t9112: f64) -> f64 {
    let t29744 = 3.0_f64 * t10556 * t1535 * t1536 + 18.0_f64 * t1535 * t8758 * t8779 + 18.0_f64 * t2718 * t6758 * t9112 + t16825 + t16946 + t16950 - t20377 - t29146 - t29149 - t29150 + t29151;
    t29744
}
