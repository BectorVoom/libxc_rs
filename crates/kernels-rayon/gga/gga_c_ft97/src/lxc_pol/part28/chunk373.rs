//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 373/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk373(t1852: f64, t5731: f64, t83: f64, t5672: f64, t5689: f64, t5669: f64, t5678: f64, t5682: f64, t5686: f64, t5694: f64, t5698: f64, t5702: f64) -> (f64, f64, f64, f64) {
    let t5732 = t1852 * t5731;
    let t5733 = t83 * t5732;
    let t5737 = t5672 / 6.0_f64;
    let t5740 = t5689 / 3.0_f64;
    let t5743 = t5669 / 4.0_f64 + t5737 + t5678 / 6.0_f64 + t5682 - t5686 / 2.0_f64 + t5740 + t5694 / 3.0_f64 + 2.0_f64 * t5698 - t5702;
    (t5733, t5737, t5740, t5743)
}
