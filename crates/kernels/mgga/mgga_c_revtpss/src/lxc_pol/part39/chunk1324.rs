//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1324/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1324<F: Float>(t3: F, t31087: F, t2178: F, t2327: F, t116: F, t8273: F, t670: F, t2371: F, t8295: F, t117: F, t31066: F, t1459: F, t1461: F, t2187: F, t2189: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t8289: F, t8296: F, t8299: F) -> (F, F, F, F, F, F, F, F) {
    let t31088 = t3 * t31087;
    let t31100 = param_d * t31087;
    let t31114 = t2327 * t2178;
    let t31117 = t116 * t8273;
    let t31118 = t31117 * t670;
    let t31121 = t8295 * t2371;
    let t31124 = t117 * t31066;
    let t31127 = 12.0 * t1459 * t8296 + 6.0 * t1459 * t8299 + 6.0 * t1461 * t8289 + 6.0 * t2187 * t4162 + 3.0 * t2187 * t4165 + 3.0 * t2189 * t4158 + t31100 * t573 + 6.0 * t31114 * t572 + 12.0 * t31118 * t572 + 6.0 * t31121 * t572 + 3.0 * t31124 * t572;
    (t31088, t31100, t31114, t31117, t31118, t31121, t31124, t31127)
}
