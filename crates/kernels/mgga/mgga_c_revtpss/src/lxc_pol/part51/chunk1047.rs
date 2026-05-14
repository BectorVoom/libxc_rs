//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1047/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1047<F: Float>(t7324: F, t7950: F, t1459: F, t34007: F, t1916: F, t32366: F, t127412: F, t127480: F, t127481: F, t127483: F, t127489: F, t127490: F, t127492: F, t127495: F, t127496: F, t127498: F, t32373: F, t34011: F, t34014: F, t573: F, t5805: F, t8607: F, t8616: F) -> (F,) {
    let t127500 = t7324 * t7950;
    let t127503 = 12.0 * t1459 * t34007;
    let t127507 = 6.0 * t1916 * t32366;
    let t127508 = t127412 * t573 * param_d + 3.0 * t5805 * t8607 + t127480 + 12.0 * t127481 + 12.0 * t127483 + t127489 + 12.0 * t127490 + 6.0 * t127492 + t127495 + 6.0 * t127496 + 6.0 * t127498 + 12.0 * t127500 + t127503 + t127507 + t32373 + t34011 + t34014 + t8616;
    (t127508,)
}
