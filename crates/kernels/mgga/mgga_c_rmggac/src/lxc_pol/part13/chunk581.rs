//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 581/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk581<F: Float>(t289: F, t8048: F, t7316: F, t1249: F, t708: F, t699: F, t794: F, t1550: F, t7329: F, t7332: F, t7361: F, t7368: F, t7372: F, t7377: F, t7381: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8049 = t289 * t8048;
    let t8050 = 0.4726e1 * t8049;
    let t8056 = 0.68186654135613354325e-2 * t7316;
    let t8059 = t1249 * t708;
    let t8060 = 0.19957069503106347607e-1 * t8059;
    let t8063 = t699 * t794;
    let t8064 = t1550 * t8063;
    let t8065 = 0.11974241701863808564e0 * t8064;
    let t8066 = 0.1440846329149835838e-2 * t7329;
    let t8067 = 0.1440846329149835838e-2 * t7332;
    let t8073 = 0.72042316457491791901e-3 * t7361;
    let t8074 = 0.1702583995731913576e-4 * t7368;
    let t8075 = 0.85129199786595678799e-5 * t7372;
    let t8076 = 0.1702583995731913576e-4 * t7377;
    let t8077 = 0.5107751987195740728e-4 * t7381;
    let t8078 = t699 * t833;
    (t8050, t8056, t8060, t8063, t8065, t8066, t8067, t8073, t8074, t8075, t8076, t8077, t8078)
}
