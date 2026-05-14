//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1031/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1031<F: Float>(t10761: F, t26278: F, t1054: F, t2133: F, t8093: F, t26176: F, t37717: F, t26150: F, t37720: F, t24573: F, t26145: F, t37716: F, t571: F, t24521: F, t11780: F, t783: F, t788: F) -> (F, F, F, F, F, F, F, F) {
    let t39818 = t26278 * t10761;
    let t39821 = t2133 * t1054 * t8093;
    let t39823 = t37717 * t26176;
    let t39824 = 0.47609969197673950972e-2 * t39823;
    let t39825 = t37720 * t26150;
    let t39826 = 0.14282990759302185292e-1 * t39825;
    let t39827 = t37717 * t24573;
    let t39828 = 0.47609969197673950972e-2 * t39827;
    let t39830 = t571 * t37716 * t26145;
    let t39831 = 0.47609969197673950972e-2 * t39830;
    let t39832 = t37720 * t24521;
    let t39835 = t783 * t11780 * t788;
    (t39818, t39821, t39824, t39826, t39828, t39831, t39832, t39835)
}
