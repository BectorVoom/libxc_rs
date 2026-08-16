//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 719/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk719(t111: f64, t2098: f64, t192: f64, t531: f64, t1982: f64, t7450: f64) -> (f64, f64, f64) {
    let t24465 = t2098 * t111;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t24999 = t7450 * t111;
    (t24465, t24995, t24999)
}
