//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 503/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk503<F: Float>(t198: F, t207: F, t159: F, t215: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F, t584: F, t588: F, t20: F, t27: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1940 = t198 * t207;
    let t1941 = t215 * t159;
    let t2219 = 2.0 * t10 * t17;
    let t2221 = 8.0 * t576 * t580;
    let t2223 = 6.0 * t15 * t22;
    let t2224 = t11 * t14;
    let t2226 = 12.0 * t2224 * t22;
    let t2228 = 32.0 * t584 * t588;
    let t2230 = 20.0 * t20 * t27;
    (t1940, t1941, t2219, t2221, t2223, t2224, t2226, t2228, t2230)
}
