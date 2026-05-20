//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1348/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1348<F: Float>(t12627: F, t487: F, t1269: F, t3566: F, t1203: F, t3565: F, t12295: F, t1204: F, t3766: F, t3555: F, t3754: F, t1248: F, t3153: F) -> (F, F, F, F, F, F, F, F) {
    let t12628 = t12627 * t487;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12678 = F::cast_from(0.25925925925925925926e-1_f64) * t12295;
    let t12702 = t1204 * t3766;
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    (t12628, t12633, t12640, t12641, t12678, t12702, t12709, t12712)
}
