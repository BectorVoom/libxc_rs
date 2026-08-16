//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1002/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1002(t144: f64, t167: f64, t1901: f64, t2075: f64, t2179: f64, t2180: f64, t2185: f64, t2205: f64, t2210: f64, t3440: f64, t379: f64, t38064: f64, t38930: f64, t39658: f64, t40840: f64, t40847: f64, t446: f64, t558: f64, t569: f64, t574: f64, t616: f64, t7959: f64, t9007: f64, t9276: f64, t9311: f64, t9327: f64, t9344: f64, t9419: f64, t9462: f64) -> f64 {
    let t40880 = 8.0_f64 / 9.0_f64 * t40840 + 8.0_f64 / 3.0_f64 * t446 * t2185 * t167 * t9007 * t558 - 8.0_f64 / 9.0_f64 * t40847 - 12.0_f64 * t446 * t144 * t39658 - 4.0_f64 * t446 * t574 * t2179 * t2180 * t2075 - 8.0_f64 * t446 * t574 * t9276 * t9311 + 16.0_f64 / 9.0_f64 * t446 * t2205 * t616 * t7959 + 40.0_f64 / 27.0_f64 * t446 * t9327 * t167 * t38064 - 4.0_f64 / 9.0_f64 * t446 * t569 * t9462 * t379 - 4.0_f64 * t1901 * t2210 * t3440 * t38930 - 8.0_f64 / 3.0_f64 * t1901 * t9419 * t9344;
    t40880
}
