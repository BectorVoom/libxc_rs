//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 886/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk886(t12845: f64, t12847: f64, t12852: f64, t12855: f64, t12857: f64, t12860: f64, t12864: f64, t12869: f64, t12875: f64, t12878: f64, t12880: f64, t13173: f64, t13210: f64, t13286: f64, t1421: f64, t456: f64) -> f64 {
    let t13288 = t12845 - 0.59133867e-2_f64 * t12847 * t12852 + 0.39422578e-2_f64 * t12855 - 0.26281718666666666667e-2_f64 * t12857 + 0.39422577999999999999e-2_f64 * t1421 * t12860 + 0.59133867e-2_f64 * t1421 * t12864 - 0.39422577999999999999e-2_f64 * t1421 * t12869 - 0.59133867e-2_f64 * t456 * t12875 - 0.98556445e-3_f64 * t12878 + 0.65704296666666666665e-3_f64 * t12880 + t13173 + t13210 + t13286;
    t13288
}
