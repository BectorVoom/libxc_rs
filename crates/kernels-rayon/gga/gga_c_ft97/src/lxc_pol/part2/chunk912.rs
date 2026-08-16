//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 912/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk912(t14108: f64, t2568: f64, t242: f64, t1162: f64, t2399: f64, t89: f64, t18: f64, t505: f64, t3885: f64, t2606: f64, t3892: f64, t3891: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14109 = t2568 * t14108;
    let t14110 = t242 * t14109;
    let t14114 = t89 * t2399 * t1162;
    let t14116 = t18 * t505;
    let t14117 = t3885 * t14116;
    let t14118 = t2606 * t14117;
    let t14121 = t3892 * t14116;
    let t14122 = t3891 * t14121;
    (t14109, t14110, t14114, t14116, t14118, t14122)
}
