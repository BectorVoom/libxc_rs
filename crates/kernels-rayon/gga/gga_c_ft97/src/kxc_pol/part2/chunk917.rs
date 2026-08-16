//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 917/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk917(t10007: f64, t14192: f64, t2492: f64, t265: f64, t13702: f64, t9802: f64, t13706: f64, t2409: f64, t3869: f64, t2606: f64, t10000: f64, t10012: f64, t14156: f64, t14160: f64, t14164: f64, t14168: f64, t14172: f64, t14177: f64, t14184: f64, t14189: f64, t1901: f64, t9997: f64) -> f64 {
    let t14193 = t10007 * t14192;
    let t14196 = t2492 * t265;
    let t14197 = t14196 * t13702;
    let t14200 = t9802 * t265;
    let t14201 = t14200 * t13706;
    let t14205 = t3869 * t2409;
    let t14206 = t2606 * t14205;
    let t14209 = 2.0_f64 / 9.0_f64 * t1901 * t14156 + 2.0_f64 / 9.0_f64 * t1901 * t14160 - 4.0_f64 / 9.0_f64 * t1901 * t14164 - 2.0_f64 / 9.0_f64 * t1901 * t14168 - 2.0_f64 / 9.0_f64 * t1901 * t14172 - 4.0_f64 / 9.0_f64 * t1901 * t14177 - t9997 / 9.0_f64 + 8.0_f64 / 27.0_f64 * t10000 - 4.0_f64 / 9.0_f64 * t1901 * t14184 + 4.0_f64 / 27.0_f64 * t1901 * t14189 - 2.0_f64 / 9.0_f64 * t1901 * t14193 - 4.0_f64 / 9.0_f64 * t1901 * t14197 + 4.0_f64 / 27.0_f64 * t1901 * t14201 - 2.0_f64 / 27.0_f64 * t10012 - 2.0_f64 / 9.0_f64 * t1901 * t14206;
    t14209
}
