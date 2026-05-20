//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 932/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk932<F: Float>(t30: F, t33: F, t5549: F, t5552: F, t580: F, t605: F, t1711: F, t3841: F, t2: F, t516: F, t1113: F, t162: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5556 = piecewise3::<F>(t31, F::new(0.0), F::new(4.0) / F::new(9.0) * t5549 * t605 + F::new(8.0) / F::new(3.0) * t5552 * t580);
    let t5557 = t3841 * t1711;
    let t5560 = t516 * t2;
    let t5564 = piecewise3::<F>(t34, F::new(0.0), F::new(4.0) / F::new(9.0) * t5557 * t1113 - F::new(8.0) / F::new(3.0) * t5560 * t580);
    let t5566 = (t5556 + t5564) * t162;
    (t5557, t5560, t5566)
}
