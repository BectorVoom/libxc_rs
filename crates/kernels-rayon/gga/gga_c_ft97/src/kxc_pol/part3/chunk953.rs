//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 953/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk953(t18724: f64, t2600: f64, t2599: f64, t258: f64, t5053: f64, t684: f64, t14159: f64, t3898: f64, t13839: f64, t3870: f64, t5147: f64, t761: f64) -> (f64, f64, f64, f64, f64) {
    let t18725 = t2600 * t18724;
    let t18726 = t2599 * t18725;
    let t18729 = t258 * t5053;
    let t18730 = t18729 * t684;
    let t18731 = t2599 * t18730;
    let t18734 = t14159 * t3898;
    let t18737 = t13839 * t3870;
    let t18740 = t761 * t5147;
    (t18726, t18731, t18734, t18737, t18740)
}
