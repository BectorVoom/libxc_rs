//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 907/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk907(t4939: f64, t688: f64, t200: f64, t807: f64, t17895: f64, t2394: f64, t9609: f64, t17903: f64, t9524: f64, t173: f64, t5045: f64, t701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18010 = t4939 * t688;
    let t18011 = t18010 * t200;
    let t18012 = t807 * t18011;
    let t18015 = t2394 * t17895;
    let t18018 = t9609 * t18011;
    let t18021 = t2394 * t17903;
    let t18024 = t9524 * t18011;
    let t18031 = t173 * t5045;
    let t18032 = t701 * t18031;
    (t18010, t18012, t18015, t18018, t18021, t18024, t18032)
}
