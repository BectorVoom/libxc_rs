//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1633/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1633<F: Float>(t6075: F, t1544: F, t198: F, t207: F, t2403: F, t2411: F, t39537: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t77460: F, t87318: F, t87342: F, t87357: F, t87373: F, t87640: F, t87920: F, t892: F) -> F {
    let t87926 = t6075 * t6075;
    let t87931 = t39537 - t39540 + t39741 + t39744 + t39747 + t87318 + t39750 + t39756 + t39760 + F::cast_from(12.0_f64) * t2403 * t77460 * t1544 - t39764 + t198 * t207 * (t87342 + t87357 + t87373 + t87920) * t892 - F::cast_from(3.0_f64) * t198 * t207 * t87926 * t2411 + t87640;
    t87931
}
