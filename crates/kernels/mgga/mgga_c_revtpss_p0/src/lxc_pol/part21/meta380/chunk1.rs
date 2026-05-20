//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1793/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1793<F: Float>(t1170: F, t12233: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12379: F, t12395: F, t12408: F, t12413: F, t12417: F, t12418: F, t12423: F, t3447: F, t3472: F, t3480: F, t435: F) -> F {
    let t12426 = -F::cast_from(0.19751673498613801407e-1_f64) * t12379 - t12233 - t12240 - t12242 - t12245 + t12251 - t12360 + t12363 - t12366 + t12395 - F::new(0.310907e-1) * t12408 * t435 + t12413 - t12417 + F::new(3.0) * t12418 * t1170 + F::new(3.0) * t3447 * t3472 + F::cast_from(0.96491876992155210402e2_f64) * t12423 * t3480;
    t12426
}
