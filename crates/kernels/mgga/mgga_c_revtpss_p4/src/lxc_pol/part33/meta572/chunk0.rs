//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1981/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1981<F: Float>(t10115: F, t555: F, t4146: F, t198: F, t775: F, t13026: F, t65: F, t2246: F, t4171: F, t10308: F, t1466: F, t21661: F, t602: F) -> (F, F, F, F, F, F, F) {
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t50080 = t198 * t775;
    let t57549 = t65 * t13026;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60670 = t21661 * t602;
    (t47567, t47672, t50080, t57549, t60221, t60224, t60670)
}
