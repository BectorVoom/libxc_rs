//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1136/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1136(t1888: f64, t232: f64, t6646: f64, t81672: f64, t131: f64, t2587: f64, t81142: f64, t1905: f64, t9537: f64, t23004: f64, t23110: f64, t23185: f64) -> (f64, f64, f64, f64) {
    let t81675 = t1888 * t6646 * t81672 * t232;
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81691 = t23185 * t23110 * t23004;
    (t81675, t81686, t81688, t81691)
}
