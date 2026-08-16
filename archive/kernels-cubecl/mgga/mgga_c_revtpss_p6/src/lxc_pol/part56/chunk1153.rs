//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1153/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1153<F: Float>(t1916: F, t32369: F, t28264: F, t572: F, t7741: F, t5795: F, t8611: F, t1459: F, t34012: F, t32375: F, t1518: F, t670: F, t8460: F) -> (F, F, F, F, F, F) {
    let t127442 = F::cast_from(12.0_f64) * t1916 * t32369;
    let t127447 = F::cast_from(12.0_f64) * t572 * t28264 * t7741;
    let t127449 = F::cast_from(6.0_f64) * t5795 * t8611;
    let t127453 = F::cast_from(6.0_f64) * t1459 * t34012;
    let t127455 = F::cast_from(6.0_f64) * t1916 * t32375;
    let t127459 = F::cast_from(6.0_f64) * t572 * t670 * t8460 * t1518;
    (t127442, t127447, t127449, t127453, t127455, t127459)
}
