//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1303/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1303(t1342: f64, t2416: f64, t49581: f64, t13900: f64, t4814: f64, t7669: f64, t56957: f64, t57113: f64, t57117: f64, t57120: f64, t57185: f64, t57213: f64, t57215: f64, t57217: f64, t57219: f64, t57222: f64) -> (f64, f64, f64) {
    let t57225 = 0.64327297288604419288e2_f64 * t2416 * t49581 * t1342;
    let t57228 = 0.3103500882342370105e4_f64 * t7669 * t13900 * t4814;
    let t57229 = t56957 + t57113 + t57117 + t57120 - t57185 + t57213 + t57215 + t57217 + t57219 - t57222 + t57225 + t57228;
    (t57225, t57228, t57229)
}
