//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 146/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk146<F: Float>(t556: F, t561: F, t213: F, t149: F, t198: F, t522: F, t524: F, t532: F) -> (F, F, F) {
    let t562 = t556 * t561;
    let t565 = F::cast_from(1.0_f64) + F::cast_from(0.65854491829355115987e0_f64) * t213 * t562;
    let t566 = F::ln(t565);
    let t569 = t198 * t532 * t566 - t149 + t522 + t524;
    (t565, t566, t569)
}
