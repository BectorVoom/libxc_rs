//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2138/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2138<F: Float>(t18498: F, t27159: F, t1468: F, t4537: F, t106546: F, t106555: F, t106562: F, t106566: F, t106569: F, t1940: F, t2403: F, t25206: F, t25440: F, t27158: F, t27364: F, t27368: F, t27382: F, t27395: F, t27402: F, t29592: F, t29606: F, t29713: F, t29719: F, t50080: F, t7087: F, t7091: F, t7749: F, t7783: F, t93404: F) -> F {
    let t106572 = t27159 * t18498;
    let t106583 = t1468 * t4537;
    let t106588 = F::new(6.0) * t27158 * t106546 + F::new(3.0) * t50080 * t29592 + F::new(3.0) * t2403 * t27364 * t7749 + F::new(2.0) * t27382 * t106555 - t1940 * t25440 * t29719 / F::new(2.0) + F::new(3.0) * t25206 * t106562 - F::new(3.0) * t27382 * t106566 - F::new(3.0) * t27158 * t106569 + F::new(6.0) * t27158 * t106572 + F::new(3.0) * t2403 * t7783 * t27395 + t1940 * t93404 * t29713 + F::new(3.0) / F::new(2.0) * t2403 * t7087 * t29606 - t1940 * t7091 * t106583 - t1940 * t27368 * t27402;
    t106588
}
