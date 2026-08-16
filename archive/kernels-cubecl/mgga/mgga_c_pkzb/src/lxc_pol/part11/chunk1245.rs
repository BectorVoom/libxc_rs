//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1245/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1245<F: Float>(t17487: F, t20754: F, t21055: F, t21058: F, t21059: F, t30314: F, t30316: F, t30319: F, t30322: F, t30324: F, t30326: F, t30328: F, t30331: F, t30338: F, t30342: F, t30346: F, t30350: F, t30353: F, t30356: F) -> F {
    let t30587 = F::cast_from(0.94674375e0_f64) * t30314 + F::cast_from(0.94674375e0_f64) * t30316 + F::cast_from(0.31558125e0_f64) * t30319 - F::cast_from(0.6618234375e1_f64) * t30322 + F::cast_from(0.794188125e1_f64) * t30324 - F::cast_from(0.52945875e1_f64) * t30326 - F::cast_from(0.52945875e1_f64) * t30328 - F::cast_from(0.17648625e1_f64) * t30331 + t21055 + t21058 + t21059 - F::cast_from(0.27785333333333333333e1_f64) * t20754 + t17487 + F::cast_from(0.937755e0_f64) * t30338 + F::cast_from(0.312585e0_f64) * t30342 + F::cast_from(0.312585e0_f64) * t30346 + F::cast_from(0.937755e0_f64) * t30350 - F::cast_from(0.62517e0_f64) * t30353 - F::cast_from(0.20839e0_f64) * t30356;
    t30587
}
