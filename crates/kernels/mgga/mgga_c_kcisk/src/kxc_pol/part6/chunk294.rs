//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 294/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk294<F: Float>(t1634: F, t583: F, t573: F, t574: F) -> (F, F, F, F, F) {
    let t1635 = F::cast_from(0.17808333333333333333e-1_f64) * t1634;
    let t1643 = t583 * t583;
    let t1644 = F::cast_from(1.0_f64) / t1643;
    let t1645 = t573 * t1644;
    let t1646 = F::cast_from(1.0_f64) / t574;
    (t1635, t1643, t1644, t1645, t1646)
}
