//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 795/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk795<F: Float>(t5: F, t10296: F, t10298: F, t10301: F, t10309: F, t10310: F, t10313: F, t10410: F, t2242: F, t2247: F, t2248: F, t2315: F, t603: F, t644: F, t91: F, t117: F, t116: F, t2319: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t10414 = piecewise3(t8, 0.0, t10296 * t91 - 12.0 * t10298 * t644 + 60.0 * t10301 * t2248 - 120.0 * t10309 * t10310 + 60.0 * t10313 * t2247 - 4.0 * t10410 * t603 - 12.0 * t2242 * t2315);
    let t10415 = t10414 * t117;
    let t10416 = t2319 * t116;
    (t10414, t10415, t10416)
}
