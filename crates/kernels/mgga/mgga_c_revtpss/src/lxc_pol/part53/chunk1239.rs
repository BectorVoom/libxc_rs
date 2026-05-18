//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1239/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1239<F: Float>(t7334: F, t8245: F, t7331: F, t127465: F, t127468: F, t127472: F, t127475: F, t127480: F, t127481: F, t127483: F, t129523: F, t573: F, t5802: F, t8771: F) -> F {
    let t129541 = t8245 * t7334;
    let t129543 = t8245 * t7331;
    let t129552 = t129523 * t573 * param_d + F::new(6.0) * t5802 * t8771 + t127465 + t127468 + t127472 + F::new(6.0) * t127475 + t127480 + F::new(6.0) * t127481 + F::new(6.0) * t127483 + F::new(3.0) * t129541 + F::new(6.0) * t129543;
    t129552
}
