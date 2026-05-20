//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2043/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2043<F: Float>(t670: F, t7968: F, t102019: F, t109150: F, t109368: F, t110054: F, t110110: F, t1312: F, t1518: F, t18245: F, t2055: F, t21881: F, t26399: F, t28653: F, t28658: F, t34251: F, t4292: F, t5920: F, t7359: F, t7373: F, t75439: F, t85360: F) -> (F, F) {
    let t111018 = t7968 * t670;
    let t111039 = F::new(4.0) * t102019 * t1518 + F::new(4.0) * t109150 * t2055 + F::new(2.0) * t109368 * t1312 + F::new(2.0) * t110110 * t670 + F::new(4.0) * t111018 * t1518 + F::new(2.0) * t18245 * t7373 + F::new(2.0) * t2055 * t75439 + F::new(2.0) * t2055 * t85360 + F::new(2.0) * t21881 * t7359 + F::new(2.0) * t26399 * t5920 + F::new(4.0) * t28653 * t4292 + F::new(2.0) * t28658 * t5920 + F::new(4.0) * t34251 * t4292 + t110054;
    (t111018, t111039)
}
