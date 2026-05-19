//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1070/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1070<F: Float>(t3727: F, t460: F, t12295: F, t1284: F, t3552: F, t1204: F, t3766: F, t3555: F, t3754: F, t1248: F, t3153: F, t3588: F, t5464: F) -> (F, F, F, F, F, F, F) {
    let t12673 = t460 * t3727;
    let t12678 = F::cast_from(0.25925925925925925926e-1_f64) * t12295;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12713 = t5464 * t3588;
    (t12673, t12678, t12699, t12702, t12709, t12712, t12713)
}
