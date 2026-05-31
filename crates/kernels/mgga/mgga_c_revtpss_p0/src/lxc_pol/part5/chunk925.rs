//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 925/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk925<F: Float>(t4049: F, t6836: F, t1394: F, t6816: F, t1877: F, t1879: F, t539: F, t541: F, t6832: F, t543: F) -> (F, F, F, F) {
    let t6837 = t4049 * t6836;
    let t6840 = t1394 * t6816;
    let t6843 = F::cast_from(6.0_f64) * t1877 * t1879 - F::cast_from(12.0_f64) * t539 * t6837 + F::cast_from(3.0_f64) * t539 * t6840 - t541 * t6832;
    let t6844 = t6843 * t543;
    (t6837, t6840, t6843, t6844)
}
