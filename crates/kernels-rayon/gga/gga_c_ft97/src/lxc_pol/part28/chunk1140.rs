//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1140/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1140(t1369: f64, t34839: f64, t376: f64, t5842: f64, t6615: f64, t7239: f64, t7366: f64, t7369: f64, t139431: f64, t147730: f64, t32897: f64, t148284: f64, t23657: f64, t23667: f64) -> (f64, f64, f64, f64, f64) {
    let t148449 = t1369 * t376 * t34839;
    let t148451 = t5842 * t6615;
    let t148454 = t7366 * t7239 * t7369 * t148451;
    let t148457 = t32897 * t139431 * t147730;
    let t148460 = t23657 * t23667 * t148284;
    (t148449, t148451, t148454, t148457, t148460)
}
