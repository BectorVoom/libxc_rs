//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 753/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk753<F: Float>(t1907: F, t615: F, t1745: F, t589: F, t7312: F, t8478: F, t8492: F, t8529: F, t8558: F, t8572: F, t8578: F, t9176: F, t9178: F, t9186: F, t9190: F, t9191: F, t9196: F, t9198: F, t9199: F, t9202: F, t9206: F, t9211: F) -> (F, F) {
    let t9517 = t615 * t1907;
    let t9522 = t589 * t1745;
    let t9528 = t9176 + t7312 - t9178 + 0.62896184579208304136e-3 * t8478 + 0.62896184579208304135e-3 * t8492 - t9186 - t9190 + t9191 + 0.85748036236139473944e-3 * t9522 - t9196 + 0.31448092289604152068e-3 * t8529 + t9198 - t9199 - t9202 + t9206 - 0.62896184579208304136e-3 * t8558 - 0.41930789719472202757e-3 * t8572 - t9211 + 0.94344276868812456204e-3 * t8578;
    (t9517, t9528)
}
