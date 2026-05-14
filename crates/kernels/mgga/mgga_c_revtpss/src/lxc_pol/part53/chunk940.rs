//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 940/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk940<F: Float>(t30: F, t7782: F, t1468: F, t1940: F, t2403: F, t31863: F, t31876: F, t33727: F, t7091: F, t7749: F, t7787: F, t8490: F, t8494: F, t1544: F, t1583: F, t198: F, t207: F, t33726: F, t892: F) -> (F, F, F) {
    let t33740 = t30 * t7782;
    let t33748 = 3.0 / 2.0 * t2403 * t8490 * t7749 + t1940 * t33727 * t30 / 2.0 - t1940 * t31863 * t7787 / 2.0 + t1940 * t8490 * t1468 / 2.0 - 3.0 / 2.0 * t2403 * t8494 * t7749 - t1940 * t7091 * t33740 + t1940 * t31876 * t7787 - t1940 * t8494 * t1468 / 2.0;
    let t33866 = t198 * t207 * t33726 * t892 + 3.0 * t1544 * t2403 * t8490 - 3.0 * t1544 * t2403 * t8494 - t1583 * t1940 * t31863 + 2.0 * t1583 * t1940 * t31876 - 2.0 * t1940 * t7091 * t7782;
    (t33740, t33748, t33866)
}
