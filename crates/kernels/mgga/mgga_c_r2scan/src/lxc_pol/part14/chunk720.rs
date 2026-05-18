//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 720/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk720<F: Float>(t5673: F, t188: F, t5448: F, t5672: F, t1893: F, t644: F, t1647: F, t1891: F, t4831: F, t4832: F, t4833: F, t4834: F, t4835: F, t5309: F, t5312: F, t5315: F) -> (F, F, F) {
    let t5674 = F::new(1.0) / t5673;
    let t5675 = t188 * t5674;
    let t5678 = F::new(0.24955700379505800916e5) * t5672 * t5675 * t5448;
    let t5679 = t644 * t1893;
    let t5682 = F::new(0.1551780387578202009e4) * t1891 * t5679 * t1647;
    let t5686 = -F::new(0.126595e1) * t5309 + F::new(0.84396666666666666667e0) * t5312 - F::new(0.3938511111111111111e1) * t5315 - t4831 + t4832 - t4833 - t4834 - t4835;
    (t5678, t5682, t5686)
}
