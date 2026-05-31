//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1185/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1185<F: Float>(t11006: F, t256: F, t10115: F, t251: F, t2410: F, t3335: F, t11198: F, t340: F, t11119: F, t384: F, t11238: F, t196: F) -> (F, F, F, F, F, F, F) {
    let t41077 = F::cast_from(1.0_f64) / t11006 / t256;
    let t41117 = t10115 * t251;
    let t41153 = t2410 * t2410;
    let t41154 = F::cast_from(1.0_f64) / t41153;
    let t41936 = t3335 * t3335;
    let t41937 = F::cast_from(1.0_f64) / t41936;
    let t42058 = F::cast_from(1.0_f64) / t11198 / t340;
    let t42066 = F::cast_from(1.0_f64) / t11119 / t384;
    let t42859 = F::cast_from(1.0_f64) / t11238 / t196;
    (t41077, t41117, t41154, t41937, t42058, t42066, t42859)
}
