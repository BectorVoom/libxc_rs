//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1079/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1079(t255: f64, t42163: f64, t10081: f64, t10093: f64, t14080: f64, t14081: f64, t1901: f64, t242: f64, t2599: f64, t2600: f64, t2609: f64, t3891: f64, t3892: f64, t41421: f64, t42374: f64, t42376: f64, t42385: f64, t42392: f64, t42394: f64, t42399: f64, t42404: f64, t446: f64, t713: f64, t8608: f64, t9787: f64) -> f64 {
    let t42409 = t42163 * t255;
    let t42414 = 4.0_f64 / 3.0_f64 * t1901 * t9787 * t10093 + 8.0_f64 / 9.0_f64 * t42374 - 8.0_f64 / 3.0_f64 * t1901 * t42376 * t10081 + 4.0_f64 / 9.0_f64 * t1901 * t2599 * t2600 * t8608 * t713 + 4.0_f64 / 3.0_f64 * t1901 * t42385 * t2609 - 4.0_f64 / 3.0_f64 * t446 * t242 * t41421 + 8.0_f64 / 9.0_f64 * t42392 + 8.0_f64 / 3.0_f64 * t1901 * t2599 * t3892 * t42394 - 8.0_f64 / 27.0_f64 * t1901 * t3891 * t3892 * t42399 - 20.0_f64 / 27.0_f64 * t1901 * t14080 * t14081 * t42404 + 40.0_f64 / 81.0_f64 * t1901 * t42409 * t14081 * t42394;
    t42414
}
