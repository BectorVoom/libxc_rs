//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 587/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk587<F: Float>(t7721: F, t7723: F, t7725: F, t7727: F, t7734: F, t7739: F, t7743: F, t7747: F, t7752: F, t2231: F, t290: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8177 = 0.1702583995731913576e-4 * t7721;
    let t8178 = 0.5107751987195740728e-4 * t7723;
    let t8179 = 0.5107751987195740728e-4 * t7725;
    let t8180 = 0.1702583995731913576e-4 * t7727;
    let t8181 = 0.638468998399467591e-4 * t7734;
    let t8182 = 0.5107751987195740728e-4 * t7739;
    let t8183 = 0.15323255961587222184e-3 * t7743;
    let t8184 = 0.5107751987195740728e-4 * t7747;
    let t8187 = 0.212822999466489197e-4 * t7752;
    let t8188 = t290 * t2231;
    (t8177, t8178, t8179, t8180, t8181, t8182, t8183, t8184, t8187, t8188)
}
