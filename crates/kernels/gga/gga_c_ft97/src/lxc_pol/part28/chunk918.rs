//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 918/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk918<F: Float>(t1541: F, t1642: F, t3052: F, t32139: F, t136488: F, t136807: F, t136812: F, t136822: F, t136827: F, t136831: F, t136841: F, t137028: F, t145501: F, t22819: F, t25770: F, t25838: F, t3076: F, t32140: F, t32174: F, t32279: F, t32318: F, t34434: F, t378: F, t7206: F, t92596: F, t938: F) -> (F, F) {
    let t145516 = t32139 * t1541 * t1642 * t3052;
    let t145531 = 0.44455354858818847408e-2 * t92596 * t145501 - 0.4918426195414944614e-6 * t136831 * t7206 * t25770 - 0.35216694699248286686e-1 * t136488 * t32140 * t378 * t25838 + 0.78259321553885081522e-2 * t32279 * t145516 + 0.5449325310689079815e-2 * t22819 * t32174 * t34434 + 0.51074886703703703704e-1 * t136807 - 0.25537443351851851852e-1 * t136812 + 0.68099848938271604939e-1 * t136822 + 0.78259321553885081522e-2 * t136827 + 0.26086440517961693841e-2 * t136841 - 0.1443087735596363459e-7 * t3076 * t137028 * t32318 * t938;
    (t145516, t145531)
}
