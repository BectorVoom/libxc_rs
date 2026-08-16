//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 910/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk910(t18063: f64, t701: f64, t3799: f64, t3810: f64, t3807: f64, t13616: f64, t17780: f64, t17727: f64, t2320: f64, t17732: f64, t3806: f64, t172: f64, t228: f64, t231: f64, t4995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18064 = t701 * t18063;
    let t18066 = t3799 * t3810;
    let t18068 = t3799 * t3807;
    let t18070 = t13616 * t17780;
    let t18071 = t701 * t18070;
    let t18073 = t2320 * t17727;
    let t18074 = t701 * t18073;
    let t18076 = t3806 * t17732;
    let t18077 = t701 * t18076;
    let t18081 = t228 * t4995 * t172 * t231;
    (t18064, t18066, t18068, t18071, t18074, t18077, t18081)
}
