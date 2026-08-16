//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 721/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk721(t111: f64, t2098: f64, t2274: f64, t50: f64, t7245: f64, t9239: f64, t2127: f64, t23383: f64) -> (f64, f64, f64, f64) {
    let t24465 = t2098 * t111;
    let t24498 = t50 * t2274;
    let t24514 = t9239 * t7245;
    let t24574 = t2127 * t23383;
    (t24465, t24498, t24514, t24574)
}
