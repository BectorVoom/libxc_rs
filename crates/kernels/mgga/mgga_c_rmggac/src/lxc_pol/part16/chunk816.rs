//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 816/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk816<F: Float>(t2186: F, t8592: F, t2320: F, t34902: F, t7414: F, t8616: F, t35584: F, t35587: F, t35591: F, t39850: F, t7229: F, t109: F, t24890: F) -> (F, F, F, F, F, F, F, F) {
    let t40121 = t2186 * t8592;
    let t40123 = t34902 * t2320;
    let t40125 = t7414 * t8616;
    let t40127 = F::new(0.5854073720911195298e0) * t35584;
    let t40128 = F::new(0.8781110581366792947e0) * t35587;
    let t40129 = F::new(0.2927036860455597649e0) * t35591;
    let t40145 = t7229 * t39850;
    let t40167 = t24890 * t109;
    (t40121, t40123, t40125, t40127, t40128, t40129, t40145, t40167)
}
