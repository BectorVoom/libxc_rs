//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 151/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk151<F: Float>(t225: F, t555: F, t546: F, t213: F) -> (F, F, F) {
    let t556 = t555 * t225;
    let t557 = t546 * t555;
    let t560 = F::new(1.0) + F::cast_from(0.65854491829355115987e0_f64) * t213 * t557;
    let t561 = F::new(1.0) / t560;
    (t556, t560, t561)
}
