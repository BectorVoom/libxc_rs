//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 867/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk867(t34808: f64, t7369: f64, t32888: f64, t7239: f64, t32898: f64, t32899: f64, t925: f64, t32897: f64, t1017: f64, t7312: f64) -> (f64, f64, f64, f64, f64) {
    let t34809 = t7369 * t34808;
    let t34811 = t32888 * t7239 * t34809;
    let t34814 = t32898 * t32899 * t925;
    let t34815 = t32897 * t34814;
    let t34817 = t7312 * t1017;
    (t34809, t34811, t34814, t34815, t34817)
}
