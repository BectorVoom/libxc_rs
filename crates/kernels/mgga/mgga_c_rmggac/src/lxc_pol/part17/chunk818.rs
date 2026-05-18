//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 818/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk818<F: Float>(t2320: F, t34902: F, t7414: F, t8616: F, t39850: F, t7229: F, t109: F, t24890: F, t490: F, t5011: F, t511: F, t270: F, t38843: F, t7349: F, t7351: F) -> (F, F, F, F, F, F) {
    let t40123 = t34902 * t2320;
    let t40124 = F::new(0.24829349937757072982e-4) * t40123;
    let t40125 = t7414 * t8616;
    let t40126 = F::new(0.24829349937757072982e-4) * t40125;
    let t40145 = t7229 * t39850;
    let t40167 = t24890 * t109;
    let t40168 = t490 * t40167;
    let t40193 = t5011 * t511;
    let t40198 = t7349 * t7351 * t38843 * t270;
    (t40124, t40126, t40145, t40168, t40193, t40198)
}
