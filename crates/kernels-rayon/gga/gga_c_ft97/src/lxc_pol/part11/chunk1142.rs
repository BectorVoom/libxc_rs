//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1142/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1142(t10471: f64, t8392: f64, t10510: f64, t824: f64, t9571: f64, t10448: f64, t10453: f64, t10495: f64, t2349: f64, t2801: f64, t10447: f64, t10452: f64, t10485: f64, t10492: f64, t15290: f64, t1901: f64, t2682: f64, t2874: f64, t2881: f64, t4139: f64, t4140: f64, t42404: f64, t4265: f64, t43938: f64) -> (f64, f64, f64) {
    let t43986 = t8392 * t10471;
    let t43992 = t8392 * t10510;
    let t43999 = t9571 * t824;
    let t44007 = t8392 * t10448;
    let t44009 = t8392 * t10453;
    let t44011 = t8392 * t10495;
    let t44013 = t2349 * t2801;
    let t44021 = -8.0_f64 / 9.0_f64 * t43986 - 4.0_f64 * t1901 * t2881 * t4140 * t42404 + 8.0_f64 / 9.0_f64 * t43992 + 8.0_f64 / 3.0_f64 * t1901 * t10492 * t4265 * t2349 * t2682 + 8.0_f64 / 3.0_f64 * t1901 * t2874 * t4140 * t43999 - 8.0_f64 / 3.0_f64 * t1901 * t10447 * t10452 - 8.0_f64 / 9.0_f64 * t44007 + 8.0_f64 / 9.0_f64 * t44009 + 8.0_f64 / 9.0_f64 * t44011 + 4.0_f64 / 9.0_f64 * t1901 * t4139 * t10485 * t44013 + 8.0_f64 / 9.0_f64 * t1901 * t15290 * t43938;
    (t43999, t44013, t44021)
}
