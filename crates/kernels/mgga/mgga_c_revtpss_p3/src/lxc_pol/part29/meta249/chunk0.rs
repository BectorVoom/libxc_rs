//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1029/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1029<F: Float>(t30: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t2: F, t513: F, t580: F, t605: F, t1711: F, t3841: F, t516: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t5545 = t1317 * t1857;
    let t5546 = F::cast_from(4.0_f64) * t5545;
    let t5547 = t1320 * t1857;
    let t5548 = F::cast_from(4.0_f64) * t5547;
    let t5549 = t3833 * t1468;
    let t5552 = t513 * t2;
    let t5556 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5549 * t605 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5552 * t580);
    let t5557 = t3841 * t1711;
    let t5560 = t516 * t2;
    (t5546, t5548, t5549, t5552, t5556, t5557, t5560)
}
