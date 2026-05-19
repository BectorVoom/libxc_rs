//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1079/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1079<F: Float>(t33574: F, t8085: F, t7987: F, t9159: F, t2226: F, t33802: F, t2131: F, t2132: F, t2385: F, t847: F, t2230: F, t33429: F) -> (F, F, F, F, F) {
    let t38455 = t33574 * t8085;
    let t38458 = F::cast_from(0.34694512752820797848e1_f64) * t7987 * t9159;
    let t38471 = F::cast_from(0.17347256376410398924e1_f64) * t33802 * t2226;
    let t38474 = t2131 * t2132 * t2385 * t847;
    let t38481 = F::cast_from(0.17347256376410398924e1_f64) * t33429 * t2230;
    (t38455, t38458, t38471, t38474, t38481)
}
