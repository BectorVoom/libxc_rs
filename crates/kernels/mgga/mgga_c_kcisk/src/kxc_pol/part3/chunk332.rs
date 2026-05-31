//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 332/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk332<F: Float>(t1635: F, t1638: F, t587: F, t583: F, t573: F, t574: F) -> (F, F, F, F, F, F) {
    let t1640 = -t1635 - F::cast_from(0.17808333333333333333e-1_f64) * t1638;
    let t1642 = F::cast_from(0.62182e-1_f64) * t1640 * t587;
    let t1643 = t583 * t583;
    let t1644 = F::cast_from(1.0_f64) / t1643;
    let t1645 = t573 * t1644;
    let t1646 = F::cast_from(1.0_f64) / t574;
    (t1640, t1642, t1643, t1644, t1645, t1646)
}
