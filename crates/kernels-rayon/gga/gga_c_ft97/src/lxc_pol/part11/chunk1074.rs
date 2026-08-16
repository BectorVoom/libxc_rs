//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1074/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1074(t240: f64, t9577: f64, t1526: f64, t9483: f64, t9499: f64, t15567: f64, t17687: f64, t17694: f64, t2320: f64, t3806: f64, t42264: f64, t42267: f64, t42270: f64, t42273: f64, t9490: f64, t9514: f64, t9571: f64, t9583: f64, t9592: f64, t9775: f64) -> f64 {
    let t42279 = t240 * t9577;
    let t42288 = t1526 * t9483 * t9499;
    let t42290 = -t1526 * t2320 * t9490 * t9571 / 2.0_f64 + t15567 * t17694 * t9592 / 2.0_f64 + t42264 / 18.0_f64 - t42267 / 6.0_f64 - t42270 / 12.0_f64 - t42273 / 9.0_f64 + 2.0_f64 * t9514 + t1526 * t2320 * t9775 / 2.0_f64 + 2.0_f64 / 3.0_f64 * t1526 * t3806 * t42279 * t9571 - t15567 * t17687 * t9583 / 3.0_f64 + t42288 / 6.0_f64;
    t42290
}
