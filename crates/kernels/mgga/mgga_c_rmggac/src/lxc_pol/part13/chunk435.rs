//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 435/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk435<F: Float>(t202: F, t4443: F, t1228: F, t1237: F, t3046: F, t31: F, t212: F, t222: F, t1224: F, t28: F, t492: F, t1233: F) -> (F, F, F, F, F, F, F) {
    let t4444 = t202 * t4443;
    let t4451 = t1228 * t1237;
    let t4457 = t31 * t3046;
    let t4460 = F::cast_from(0.92481467875469997376e0_f64) * t212 * t4457 * t222;
    let t4461 = t1224 * t28;
    let t4462 = t212 * t4461;
    let t4463 = t4462 * t492;
    let t4465 = t1228 * t1233;
    (t4444, t4451, t4460, t4461, t4462, t4463, t4465)
}
