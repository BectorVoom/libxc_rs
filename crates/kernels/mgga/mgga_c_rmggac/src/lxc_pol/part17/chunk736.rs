//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 736/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk736<F: Float>(t40088: F, t2001: F, t2281: F, t326: F, t333: F, t2186: F, t8592: F, t2320: F, t34902: F, t7414: F, t8616: F, t39850: F, t7229: F, t109: F, t24890: F, t490: F) -> (F, F, F, F, F, F, F) {
    let t40089 = 0.19211284388664477842e-2 * t40088;
    let t40092 = t2001 * t326 * t2281 * t333;
    let t40121 = t2186 * t8592;
    let t40123 = t34902 * t2320;
    let t40124 = 0.24829349937757072982e-4 * t40123;
    let t40125 = t7414 * t8616;
    let t40126 = 0.24829349937757072982e-4 * t40125;
    let t40145 = t7229 * t39850;
    let t40167 = t24890 * t109;
    let t40168 = t490 * t40167;
    (t40089, t40092, t40121, t40124, t40126, t40145, t40168)
}
