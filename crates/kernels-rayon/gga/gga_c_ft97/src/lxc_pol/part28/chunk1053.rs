//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1053/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1053(t1541: f64, t1642: f64, t3052: f64, t32139: f64, t136488: f64, t136807: f64, t136812: f64, t136822: f64, t136827: f64, t136831: f64, t136841: f64, t137028: f64, t145501: f64, t22819: f64, t25770: f64, t25838: f64, t3076: f64, t32140: f64, t32174: f64, t32279: f64, t32318: f64, t34434: f64, t378: f64, t7206: f64, t92596: f64, t938: f64) -> (f64, f64) {
    let t145516 = t32139 * t1541 * t1642 * t3052;
    let t145531 = 0.44455354858818847408e-2_f64 * t92596 * t145501 - 0.4918426195414944614e-6_f64 * t136831 * t7206 * t25770 - 0.35216694699248286686e-1_f64 * t136488 * t32140 * t378 * t25838 + 0.78259321553885081522e-2_f64 * t32279 * t145516 + 0.5449325310689079815e-2_f64 * t22819 * t32174 * t34434 + 0.51074886703703703704e-1_f64 * t136807 - 0.25537443351851851852e-1_f64 * t136812 + 0.68099848938271604939e-1_f64 * t136822 + 0.78259321553885081522e-2_f64 * t136827 + 0.26086440517961693841e-2_f64 * t136841 - 0.1443087735596363459e-7_f64 * t3076 * t137028 * t32318 * t938;
    (t145516, t145531)
}
