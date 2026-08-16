//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 709/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk709(t12645: f64, t213: f64, t1011: f64, t1018: f64, t12454: f64, t12460: f64, t12462: f64, t12500: f64, t12503: f64, t12505: f64, t12631: f64, t12637: f64, t12641: f64, t12644: f64, t139: f64, t172: f64, t175: f64, t197: f64, t198: f64, t3194: f64, t3203: f64, t3209: f64, t3213: f64, t3220: f64) -> f64 {
    let t12646 = t12645 * t213;
    let t12649 = 0.74295e-1_f64 * t12454 * t3209 + 0.4953e-1_f64 * t3194 * t3213 - 0.619125e-2_f64 * t12460 * t12462 - 0.619125e-2_f64 * t197 * t12500 + 0.371475e-1_f64 * t12503 * t12505 - 0.23583209876543209876e-1_f64 * t139 * t172 * t175 + 0.619125e-2_f64 * t12631 * t198 - 0.1857375e-1_f64 * t1011 * t3220 + 0.619125e-2_f64 * t12637 * t3203 - 0.371475e-1_f64 * t12641 * t1018 + 0.41275e-2_f64 * t12644 * t12646;
    t12649
}
