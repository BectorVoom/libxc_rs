//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1907/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1907<F: Float>(t102315: F, t25899: F, t2439: F, t8099: F, t94391: F, t102234: F, t3916: F, t25895: F, t2097: F, t9990: F, t102115: F, t7289: F) -> (F, F, F, F, F, F, F) {
    let t102378 = t25899 * t102315;
    let t102385 = t8099 * t2439;
    let t102386 = t94391 * t102385;
    let t102394 = t102234 * t3916;
    let t102396 = F::cast_from(0.28912093960683998208e-1_f64) * t25895 * t102394;
    let t102397 = t9990 * t2097;
    let t102404 = F::cast_from(0.25702851531048074406e-1_f64) * t7289 * t102115;
    (t102378, t102385, t102386, t102394, t102396, t102397, t102404)
}
