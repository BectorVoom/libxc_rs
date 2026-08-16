//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 827/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk827(t18: f64, t558: f64, t2222: f64, t2221: f64, t609: f64, t2211: f64, t2210: f64, t11593: f64, t12941: f64, t12947: f64, t12952: f64, t12958: f64, t12963: f64, t12965: f64, t12967: f64, t12971: f64, t12975: f64, t12976: f64, t12979: f64, t12983: f64, t1901: f64, t28: f64, t446: f64, t89: f64, t9112: f64) -> f64 {
    let t12986 = t18 * t558;
    let t12987 = t2222 * t12986;
    let t12988 = t2221 * t12987;
    let t12991 = t18 * t609;
    let t12992 = t2211 * t12991;
    let t12993 = t2210 * t12992;
    let t12996 = t89 * t28 * t12941 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t12947 + t446 * t12952 / 3.0_f64 - 2.0_f64 / 27.0_f64 * t9112 + 2.0_f64 / 3.0_f64 * t446 * t12958 - t12963 - t12965 - t12967 - 4.0_f64 / 3.0_f64 * t1901 * t12971 - t12975 + 2.0_f64 / 9.0_f64 * t1901 * t12976 + 4.0_f64 / 9.0_f64 * t1901 * t12979 - 4.0_f64 / 27.0_f64 * t1901 * t12983 + 4.0_f64 / 9.0_f64 * t11593 * t12988 + 4.0_f64 / 9.0_f64 * t11593 * t12993;
    t12996
}
