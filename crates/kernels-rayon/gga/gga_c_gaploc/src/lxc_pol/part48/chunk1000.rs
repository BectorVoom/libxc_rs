//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 1000/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk1000(t224: f64, t44670: f64, t44671: f64, t44674: f64, t44676: f64, t44678: f64, t44679: f64, t44681: f64, t44684: f64, t44687: f64, t44689: f64, t44692: f64, t44694: f64, t44704: f64, t44705: f64, t44706: f64, t45123: f64, t45126: f64, t45130: f64, t45132: f64, t45161: f64, t45994: f64, t46836: f64) -> f64 {
    let t46840 = t44670 - t44671 - t44674 + t44676 - t44678 - t44679 + t44681 - t44684 + t44687 - t44689 + t44692 - t44694 + t224 * (t44706 + t45161 + t45994 + t46836) - t44704 - t44705 + t45123 - t45126 + t45130 - t45132;
    t46840
}
