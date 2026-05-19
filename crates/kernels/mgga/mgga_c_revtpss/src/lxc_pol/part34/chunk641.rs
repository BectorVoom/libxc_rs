//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 641/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk641<F: Float>(t1169: F, t6486: F, t3459: F, t3466: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F) -> (F, F) {
    let t6487 = t6486 * t1169;
    let t6502 = -F::new(0.17648625e1) * t6443 + F::new(0.3529725e1) * t6450 + t3459 - F::cast_from(0.34431666666666666666e0_f64) * t5044 - F::cast_from(0.34431666666666666667e0_f64) * t6423 + F::new(0.103295e1) * t6427 + F::new(0.516475e0) * t6431 + F::new(0.31558125e0) * t6456 + F::new(0.6311625e0) * t6458 + t3466 - F::cast_from(0.13892666666666666667e0_f64) * t5093 - F::cast_from(0.34731666666666666667e-1_f64) * t6462 + F::new(0.20839e0) * t6465 + F::new(0.104195e0) * t6468;
    (t6487, t6502)
}
