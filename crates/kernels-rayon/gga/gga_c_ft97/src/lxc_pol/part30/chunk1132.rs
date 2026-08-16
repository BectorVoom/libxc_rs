//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1132/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1132(t292: f64, t153020: f64, t153066: f64, t153129: f64, t153183: f64, t153229: f64, t153267: f64, t153325: f64, t153368: f64, t1486: f64, t193: f64, t852: f64, t10683: f64, t35833: f64, t446: f64, t824: f64) -> (f64, f64, f64) {
    let t293 = 0.1e-59_f64 < t292;
    let t153372 = piecewise3(t293, t153020 + t153066 + t153129 + t153183 + t153229 + t153267 + t153325 + t153368, 0.0_f64);
    let t153375 = t1486 * t193 * t852 * t153372;
    let t153379 = t446 * t10683 * t35833 * t824;
    (t153372, t153375, t153379)
}
