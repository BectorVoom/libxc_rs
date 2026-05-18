//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1058/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1058<F: Float>(t121184: F, t31805: F, t32240: F, t1385: F, t46361: F, t2470: F, t32239: F, t32238: F, t1955: F, t2681: F, t8571: F, t8575: F) -> (F, F, F, F, F) {
    let t121185 = t31805 * t121184;
    let t121186 = t121185 * t32240;
    let t121188 = t46361 * t1385;
    let t121197 = t32239 * t2470;
    let t121199 = F::new(0.19039912555034117539e-1) * t32238 * t121197;
    let t121202 = t1955 * t8571 * t2681 * t8575;
    (t121186, t121188, t121197, t121199, t121202)
}
