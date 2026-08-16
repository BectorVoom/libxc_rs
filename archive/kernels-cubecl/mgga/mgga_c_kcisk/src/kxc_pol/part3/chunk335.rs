//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 335/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk335<F: Float>(t1636: F, t1659: F, t26: F, t1638: F, t1649: F, t1651: F, t1654: F, t1658: F, t586: F) -> (F, F, F, F) {
    let t1660 = t1659 * t1636;
    let t1661 = t26 * t1660;
    let t1663 = F::cast_from(0.1898925e1_f64) * t1649 - t1651 - F::cast_from(0.29896666666666666667e0_f64) * t1638 + F::cast_from(0.3071625e0_f64) * t1654 - t1658 - F::cast_from(0.82156666666666666667e-1_f64) * t1661;
    let t1664 = F::cast_from(1.0_f64) / t586;
    (t1660, t1661, t1663, t1664)
}
