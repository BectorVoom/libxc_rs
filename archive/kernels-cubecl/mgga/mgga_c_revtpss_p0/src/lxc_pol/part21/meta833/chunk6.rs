//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3121/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3121<F: Float>(t12916: F, t17753: F, t17755: F, t12800: F, t5378: F, t17769: F, t3647: F, t1235: F, t371: F, t5318: F, t676: F, t225: F, t56331: F) -> (F, F, F, F, F) {
    let t57435 = t17753 * t12916 * t17755;
    let t57449 = t12800 * t5378;
    let t57451 = t3647 * t17769;
    let t57463 = t1235 * t371 * t676 * t5318;
    let t57464 = F::cast_from(0.14291339372689912324e-3_f64) * t57463;
    let t57465 = t56331 * t225;
    (t57435, t57449, t57451, t57464, t57465)
}
