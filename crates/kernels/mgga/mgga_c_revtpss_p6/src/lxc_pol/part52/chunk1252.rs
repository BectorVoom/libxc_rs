//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1252/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1252<F: Float>(t34028: F, t4254: F, t1518: F, t32575: F, t651: F, t28043: F, t7359: F, t34243: F, t7235: F, t34251: F, t7003: F, t125563: F, t28196: F, t28286: F) -> (F, F, F, F, F, F) {
    let t128557 = F::cast_from(2.0_f64) * t4254 * t34028;
    let t128560 = F::cast_from(2.0_f64) * t651 * t32575 * t1518;
    let t128562 = F::cast_from(2.0_f64) * t7359 * t28043;
    let t128572 = t7235 * t34243;
    let t128574 = F::cast_from(2.0_f64) * t34251 * t7003;
    let t128577 = F::cast_from(2.0_f64) * t28196 * t28286 * t125563;
    (t128557, t128560, t128562, t128572, t128574, t128577)
}
