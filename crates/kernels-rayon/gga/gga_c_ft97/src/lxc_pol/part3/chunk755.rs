//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 755/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk755(t15768: f64, t378: f64, t92: f64, t15625: f64, t358: f64, t11167: f64, t11170: f64, t11172: f64, t11177: f64, t15734: f64, t15739: f64, t15744: f64, t15748: f64, t15750: f64, t15754: f64, t15758: f64, t15760: f64, t15765: f64, t7945: f64, t7946: f64) -> (f64, f64, f64, f64) {
    let t15769 = t378 * t15768;
    let t15770 = t92 * t15769;
    let t15772 = t358 * t15625;
    let t15773 = t378 * t15772;
    let t15774 = t92 * t15773;
    let t15776 = -t7945 - 4.0_f64 / 27.0_f64 * t7946 - 8.0_f64 / 27.0_f64 * t11167 + t11170 - t11172 + 4.0_f64 / 9.0_f64 * t11177 + 2.0_f64 / 27.0_f64 * t15734 - 10.0_f64 / 27.0_f64 * t15739 + 4.0_f64 / 3.0_f64 * t15744 - 8.0_f64 / 9.0_f64 * t15748 - 2.0_f64 / 9.0_f64 * t15750 - 2.0_f64 * t15754 + 8.0_f64 / 3.0_f64 * t15758 + t15760 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t15765 + 2.0_f64 / 3.0_f64 * t15770 - t15774 / 3.0_f64;
    (t15770, t15772, t15774, t15776)
}
