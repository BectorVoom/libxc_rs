//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 576/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk576<F: Float>(t30: F, t33: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t2: F, t513: F, t580: F, t605: F, t1711: F, t3841: F, t516: F, t1113: F, t162: F, t189: F, t512: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5545 = t1317 * t1857;
    let t5546 = 4.0 * t5545;
    let t5547 = t1320 * t1857;
    let t5548 = 4.0 * t5547;
    let t5549 = t3833 * t1468;
    let t5552 = t513 * t2;
    let t5556 = piecewise3(t31, 0.0, 4.0 / 9.0 * t5549 * t605 + 8.0 / 3.0 * t5552 * t580);
    let t5557 = t3841 * t1711;
    let t5560 = t516 * t2;
    let t5564 = piecewise3(t34, 0.0, 4.0 / 9.0 * t5557 * t1113 - 8.0 / 3.0 * t5560 * t580);
    let t5566 = (t5556 + t5564) * t162;
    let t5567 = t5566 * t189;
    let t5568 = t512 * t5567;
    (t5546, t5548, t5566, t5568)
}
