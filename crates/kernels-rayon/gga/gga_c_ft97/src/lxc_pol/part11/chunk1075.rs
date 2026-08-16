//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1075/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1075(t342: f64, t657: f64, t8639: f64, t2252: f64, t2326: f64, t240: f64, t9570: f64, t630: f64, t9507: f64, t13605: f64, t1526: f64, t231: f64, t2320: f64, t2321: f64, t343: f64, t3806: f64, t8608: f64, t9512: f64, t9571: f64, t9692: f64, t9745: f64, t9757: f64, t9761: f64, t9781: f64) -> f64 {
    let t42293 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t657;
    let t42295 = t342 * t2252 * t2326;
    let t42307 = t240 * t9570;
    let t42320 = t342 * t630 * t9507;
    let t42322 = -t42293 + t9512 + t42295 / 6.0_f64 - t1526 * t2320 * t9757 / 4.0_f64 - t1526 * t2320 * t2321 * t8608 / 12.0_f64 - t1526 * t3806 * t9745 / 3.0_f64 - 7.0_f64 / 27.0_f64 * t1526 * t13605 * t42307 * t9571 - t1526 * t2320 * t9761 / 4.0_f64 - t342 * t343 * t231 * t9692 / 4.0_f64 + t9781 - t42320 / 4.0_f64;
    t42322
}
