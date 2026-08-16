//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1319/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1319(t10328: f64, t11688: f64, t23132: f64, t24081: f64, t17874: f64, t35469: f64, t11673: f64, t128: f64, t22970: f64, t24499: f64, t10346: f64, t11683: f64, t23305: f64, t2440: f64) -> (f64, f64, f64, f64, f64) {
    let t35764 = t10328 * t11688;
    let t35766 = t24081 * t23132;
    let t35768 = t35766 * t35469 * t17874;
    let t35772 = t11673 * t22970 * t128 * t24499;
    let t35776 = t10346 * t23305 * t11683 * t2440;
    (t35764, t35766, t35768, t35772, t35776)
}
