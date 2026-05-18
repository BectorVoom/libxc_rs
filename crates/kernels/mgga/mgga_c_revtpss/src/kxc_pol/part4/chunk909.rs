//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 909/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk909<F: Float>(t33: F, t1711: F, t3881: F, t1348: F, t2: F, t1113: F, t580: F, t5581: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t5582 = t3881 * t1711;
    let t5585 = t1348 * t2;
    let t5589 = piecewise3::<f64>(t34, F::new(0.0), -F::new(2.0) / F::new(9.0) * t5582 * t1113 - F::new(4.0) / F::new(3.0) * t5585 * t580);
    let t5591 = t5581 / F::new(2.0) + t5589 / F::new(2.0);
    (t5582, t5585, t5591)
}
