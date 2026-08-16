//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1175/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1175<F: Float>(t125830: F, t32705: F, t32710: F, t5710: F, t8477: F, t32272: F, t33970: F, t32275: F, t33943: F, t32279: F, t125: F, t246: F, t32276: F, t551: F, t5774: F) -> (F, F, F, F, F, F) {
    let t125831 = t32705 * t125830;
    let t125833 = t32710 * t125830;
    let t125849 = t8477 * t5710;
    let t125855 = t32272 * t33970;
    let t125867 = t33943 * t32275;
    let t125868 = t125867 * t32279;
    let t125873 = t32276 * t551 * t246 * t125 * t5774;
    (t125831, t125833, t125849, t125855, t125868, t125873)
}
