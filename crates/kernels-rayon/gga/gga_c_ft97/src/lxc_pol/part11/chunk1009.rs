//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1009/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1009(t1882: f64, t9446: f64, t2170: f64, t8232: f64, t9278: f64, t9430: f64, t2182: f64, t12746: f64, t13165: f64, t13220: f64, t1580: f64, t1901: f64, t1986: f64, t2075: f64, t2157: f64, t2180: f64, t2210: f64, t2211: f64, t2221: f64, t2222: f64, t3434: f64, t3439: f64, t3440: f64, t379: f64, t38930: f64, t38960: f64, t40772: f64, t446: f64, t616: f64, t9017: f64, t9093: f64, t9099: f64, t9121: f64, t9133: f64, t9288: f64, t9432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41123 = t1882 * t9446;
    let t41125 = t8232 * t2170;
    let t41127 = t1882 * t9278;
    let t41137 = t1882 * t9430;
    let t41139 = t8232 * t2182;
    let t41196 = -8.0_f64 * t446 * t9432 * t616 * t9017 - 8.0_f64 / 3.0_f64 * t1901 * t13220 * t9288 * t379 + 4.0_f64 / 3.0_f64 * t1901 * t9099 * t9093 - 4.0_f64 / 3.0_f64 * t1901 * t2210 * t13165 * t1580 * t2180 - 4.0_f64 / 3.0_f64 * t1901 * t9133 * t2222 * t1580 * t1986 + 8.0_f64 / 3.0_f64 * t1901 * t3439 * t12746 * t38930 + 2.0_f64 / 3.0_f64 * t1901 * t2221 * t2222 * t1580 * t2075 + 2.0_f64 / 3.0_f64 * t1901 * t2210 * t2211 * t1580 * t2157 + 8.0_f64 / 3.0_f64 * t1901 * t2210 * t9121 * t40772 + 8.0_f64 / 9.0_f64 * t1901 * t2210 * t3434 * t38960 - 8.0_f64 / 27.0_f64 * t1901 * t3439 * t3440 * t38960;
    (t41123, t41125, t41127, t41137, t41139, t41196)
}
