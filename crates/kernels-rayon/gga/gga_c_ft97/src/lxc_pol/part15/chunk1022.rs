//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1022/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1022(t11690: f64, t1787: f64, t3127: f64, t3134: f64, t38464: f64, t38478: f64, t38483: f64, t44950: f64, t462: f64, t8291: f64, t85456: f64, t85465: f64, t85474: f64, t85483: f64, t86054: f64, t86058: f64, t86068: f64, t86075: f64, t86082: f64, t86086: f64, t86090: f64, t86094: f64, t86098: f64) -> f64 {
    let t86102 = 112.0_f64 / 27.0_f64 * t44950 + 4.0_f64 / 3.0_f64 * t462 * t1787 * t86054 + 4.0_f64 / 3.0_f64 * t462 * t1787 * t86058 + 8.0_f64 / 3.0_f64 * t462 * t3134 * t85456 - 8.0_f64 / 9.0_f64 * t462 * t3127 * t85465 + 40.0_f64 / 27.0_f64 * t462 * t38483 * t86068 - 20.0_f64 / 9.0_f64 * t462 * t11690 * t85483 + 8.0_f64 * t462 * t1787 * t86075 - 12.0_f64 * t462 * t3134 * t85474 + 2.0_f64 * t462 * t1787 * t86082 - 4.0_f64 * t462 * t8291 * t86086 + 8.0_f64 * t462 * t38478 * t86090 + 8.0_f64 * t462 * t8291 * t86094 - 8.0_f64 / 3.0_f64 * t462 * t38464 * t86098;
    t86102
}
