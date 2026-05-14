//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 983/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk983<F: Float>(t39825: F, t24573: F, t37717: F, t26145: F, t37716: F, t571: F, t24521: F, t37720: F, t11780: F, t783: F, t788: F, t37754: F, t565: F, t2608: F, t37470: F, t574: F) -> (F, F, F, F, F, F, F) {
    let t39826 = 0.14282990759302185292e-1 * t39825;
    let t39827 = t37717 * t24573;
    let t39828 = 0.47609969197673950972e-2 * t39827;
    let t39830 = t571 * t37716 * t26145;
    let t39831 = 0.47609969197673950972e-2 * t39830;
    let t39832 = t37720 * t24521;
    let t39835 = t783 * t11780 * t788;
    let t39836 = 0.46574606203128791246e-1 * t39835;
    let t39840 = t565 * t37754;
    let t39846 = t574 * t37470 * t2608;
    (t39826, t39828, t39831, t39832, t39836, t39840, t39846)
}
