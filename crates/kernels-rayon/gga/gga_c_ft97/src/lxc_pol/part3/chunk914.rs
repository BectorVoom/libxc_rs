//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 914/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk914(t18117: f64, t92: f64, t17727: f64, t683: f64, t16579: f64, t668: f64, t13538: f64, t13541: f64, t13543: f64, t13544: f64, t18096: f64, t18099: f64, t18102: f64, t18105: f64, t18107: f64, t18110: f64, t18113: f64, t18115: f64, t9557: f64, t9558: f64) -> (f64, f64, f64, f64, f64) {
    let t18118 = t92 * t18117;
    let t18120 = t683 * t17727;
    let t18121 = t92 * t18120;
    let t18123 = t668 * t16579;
    let t18124 = t683 * t18123;
    let t18125 = t92 * t18124;
    let t18127 = -t9557 - 4.0_f64 / 27.0_f64 * t9558 - 8.0_f64 / 27.0_f64 * t13538 + t13541 - t13543 - 4.0_f64 / 9.0_f64 * t13544 + 2.0_f64 / 27.0_f64 * t18096 - 10.0_f64 / 27.0_f64 * t18099 + 4.0_f64 / 3.0_f64 * t18102 + 8.0_f64 / 9.0_f64 * t18105 - 2.0_f64 / 9.0_f64 * t18107 - 2.0_f64 * t18110 - 8.0_f64 / 3.0_f64 * t18113 + t18115 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t18118 + 2.0_f64 / 3.0_f64 * t18121 - t18125 / 3.0_f64;
    (t18118, t18121, t18123, t18125, t18127)
}
