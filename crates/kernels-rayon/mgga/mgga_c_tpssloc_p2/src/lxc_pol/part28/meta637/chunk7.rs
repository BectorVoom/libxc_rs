//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2039/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2039(t1398: f64, t1404: f64, t16507: f64, t1858: f64, t2105: f64, t24448: f64, t27241: f64, t3: f64, t3946: f64, t580: f64, t7946: f64, t85379: f64, t85381: f64, t85392: f64, t94106: f64, t94113: f64, t94118: f64, t94120: f64, t94122: f64, t94160: f64, t94202: f64) -> f64 {
    let t94205 = t3 * t94106 * t580 + 2.0_f64 * t27241 * t1404 + t7946 * t3946 + t94113 + t24448 * t1858 + t85392 + t16507 * t2105 + t85379 + 2.0_f64 * t85381 + t94118 + t94120 + t94122 + t1398 * (t94160 + t94202);
    t94205
}
