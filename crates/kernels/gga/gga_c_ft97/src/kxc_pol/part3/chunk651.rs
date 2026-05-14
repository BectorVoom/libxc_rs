//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 651/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk651<F: Float>(t287: F, t4061: F, t1471: F, t800: F, t13596: F, t1213: F, t1636: F, t89: F, t375: F, t4130: F, t14635: F, t14637: F, t14639: F, t14657: F, t14683: F, t1775: F, t4203: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14763 = t4061 * t287;
    let t14766 = t800 * t1471;
    let t14788 = 0.22226000364197530866e-1 * t13596;
    let t14895 = t89 * t1636 * t1213;
    let t14902 = t89 * t375 * t4130;
    let t14903 = t14902 / 9.0;
    let t14921 = 2.0 / 9.0 * t14635;
    let t14922 = 4.0 / 9.0 * t14637;
    let t14923 = 4.0 / 27.0 * t14639;
    let t14929 = 2.0 / 9.0 * t14657;
    let t14936 = 4.0 / 3.0 * t14683;
    let t14951 = 2.0 / 3.0 * t14902;
    let t14953 = 2.0 / 9.0 * t1775 * t4203;
    (t14763, t14766, t14788, t14895, t14902, t14903, t14921, t14922, t14923, t14929, t14936, t14951, t14953)
}
