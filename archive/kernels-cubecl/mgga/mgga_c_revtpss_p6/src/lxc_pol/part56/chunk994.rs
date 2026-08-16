//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 994/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk994<F: Float>(t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2403: F, t31863: F, t31876: F, t33726: F, t7091: F, t7782: F, t8490: F, t8494: F, t892: F) -> F {
    let t33866 = t198 * t207 * t33726 * t892 + F::cast_from(3.0_f64) * t1544 * t2403 * t8490 - F::cast_from(3.0_f64) * t1544 * t2403 * t8494 - t1583 * t1940 * t31863 + F::cast_from(2.0_f64) * t1583 * t1940 * t31876 - F::cast_from(2.0_f64) * t1940 * t7091 * t7782;
    t33866
}
