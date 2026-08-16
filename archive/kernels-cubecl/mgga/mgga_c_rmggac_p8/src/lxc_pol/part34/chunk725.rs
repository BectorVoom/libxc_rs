//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 725/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk725<F: Float>(t14240: F, t68524: F, t14245: F, t14229: F, t8516: F, t69179: F, t739: F, t69608: F, t7229: F, t31: F, t668: F, t640: F) -> (F, F, F, F, F, F, F) {
    let t70441 = t68524 * t14240;
    let t70443 = t68524 * t14245;
    let t70460 = t8516 * t14229;
    let t70479 = F::cast_from(0.2927036860455597649e0_f64) * t739 * t69179;
    let t70489 = t7229 * t69608;
    let t70499 = t668 * t31;
    let t70500 = t640 * t70499;
    (t70441, t70443, t70460, t70479, t70489, t70499, t70500)
}
