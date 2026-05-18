//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 886/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk886<F: Float>(t12845: F, t12847: F, t12852: F, t12855: F, t12857: F, t12860: F, t12864: F, t12869: F, t12875: F, t12878: F, t12880: F, t13173: F, t13210: F, t13286: F, t1421: F, t456: F) -> F {
    let t13288 = t12845 - F::new(0.59133867e-2) * t12847 * t12852 + F::new(0.39422578e-2) * t12855 - F::new(0.26281718666666666667e-2) * t12857 + F::new(0.39422577999999999999e-2) * t1421 * t12860 + F::new(0.59133867e-2) * t1421 * t12864 - F::new(0.39422577999999999999e-2) * t1421 * t12869 - F::new(0.59133867e-2) * t456 * t12875 - F::new(0.98556445e-3) * t12878 + F::new(0.65704296666666666665e-3) * t12880 + t13173 + t13210 + t13286;
    t13288
}
