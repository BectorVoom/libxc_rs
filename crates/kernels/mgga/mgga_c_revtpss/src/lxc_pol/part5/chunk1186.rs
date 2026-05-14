//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1186/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1186<F: Float>(t1168: F, t6487: F, t1745: F, t5142: F, t6506: F, t6503: F, t3479: F, t6502: F, t5146: F, t12472: F, t6486: F, t1130: F, t6433: F, t1151: F, t16835: F, t1733: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20606 = t6487 * t1168;
    let t20609 = t1745 * t5142;
    let t20612 = t6506 * t1168;
    let t20615 = t6503 * t1168;
    let t20618 = t6502 * t3479;
    let t20619 = t20618 * t1168;
    let t20622 = t5146 * t5142;
    let t20625 = t6486 * t12472;
    let t20626 = t20625 * t1168;
    let t20629 = t6433 * t1130;
    let t20631 = 1.0 * t20629 * t1151;
    let t20633 = 2.0 * t16835 * t1733;
    (t20606, t20609, t20612, t20615, t20619, t20622, t20626, t20631, t20633)
}
