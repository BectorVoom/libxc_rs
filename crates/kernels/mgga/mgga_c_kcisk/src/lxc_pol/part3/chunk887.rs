//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 887/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk887<F: Float>(t14190: F, t14193: F, t14195: F, t14201: F, t14206: F, t14211: F, t14216: F, t14218: F, t14220: F, t14224: F, t14226: F, t14228: F, t1203: F, t3688: F, t1197: F, t3722: F) -> (F, F, F) {
    let t14715 = 0.10446625e-1 * t14190 + 0.27857666666666666666e-1 * t14193 + 0.46429444444444444443e-2 * t14195 + 0.18571777777777777778e-1 * t14201 - 0.34822083333333333333e-2 * t14206 + 0.51588271604938271604e-3 * t14211 + 0.30952962962962962963e-2 * t14216 + 0.23214722222222222222e-2 * t14218 - 0.69644166666666666665e-2 * t14220 + 0.11607361111111111111e-2 * t14224 - 0.77382407407407407405e-3 * t14226 - 0.12381185185185185185e-1 * t14228;
    let t14728 = t3688 * t1203;
    let t14733 = t1197 * t3722;
    (t14715, t14728, t14733)
}
