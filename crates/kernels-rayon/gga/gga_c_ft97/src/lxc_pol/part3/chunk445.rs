//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 445/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk445(t432: f64, t979: f64, t452: f64, t488: f64, t492: f64, t1852: f64, t83: f64, t1882: f64, t981: f64, t986: f64, t110: f64, t3103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3214 = t979 * t432;
    let t3216 = t452 * t488 * t3214;
    let t3219 = t979 * t492;
    let t3220 = t1852 * t3219;
    let t3221 = t83 * t3220;
    let t3224 = t1882 * t981;
    let t3227 = t452 * t986 * t432;
    let t3231 = t452 * t110 * t3103;
    (t3214, t3216, t3219, t3220, t3221, t3224, t3227, t3231)
}
