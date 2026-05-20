//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1422/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1422<F: Float>(t33: F, t1711: F, t9350: F, t2: F, t3841: F, t1113: F, t580: F, t22: F, t3351: F, t3842: F, t516: F, t5557: F, t5560: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t13565 = t9350 * t1711;
    let t13568 = t3841 * t2;
    let t13569 = t580 * t1113;
    let t13579 = piecewise3::<F>(t34, F::new(0.0), -F::new(8.0) / F::new(27.0) * t13565 * t3842 - F::new(16.0) / F::new(9.0) * t13568 * t13569 + F::new(4.0) / F::new(9.0) * t5557 * t3351 - F::new(8.0) / F::new(3.0) * t516 * t580 + F::new(8.0) * t5560 * t22);
    (t13569, t13579)
}
