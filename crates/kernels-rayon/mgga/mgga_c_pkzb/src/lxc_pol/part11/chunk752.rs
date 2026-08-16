//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 752/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk752(t2241: f64, t351: f64, t6087: f64, t6174: f64, t2316: f64, t880: f64) -> (f64, f64, f64, f64) {
    let t6201 = 1.0_f64 / t2241 / t351;
    let t6211 = 0.93932222222222222223e0_f64 * t6087;
    let t6218 = 0.36793333333333333333e0_f64 * t6174;
    let t6230 = 1.0_f64 / t2316 / t880;
    (t6201, t6211, t6218, t6230)
}
