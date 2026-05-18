//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 530/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk530<F: Float>(t7218: F, t2164: F, t356: F, t638: F, t639: F, t1276: F, t640: F, t1173: F, t205: F, t671: F) -> (F, F, F, F, F, F, F) {
    let t7219 = F::new(0.15243824895787514157e-3) * t7218;
    let t7220 = t2164 * t356;
    let t7222 = t638 * t639 * t7220;
    let t7223 = F::new(0.30487649791575028314e-3) * t7222;
    let t7224 = t640 * t1276;
    let t7226 = t638 * t639 * t7224;
    let t7227 = F::new(0.15243824895787514157e-3) * t7226;
    let t7228 = t1173 * t205;
    let t7229 = t671 * t7228;
    (t7219, t7220, t7223, t7224, t7227, t7228, t7229)
}
