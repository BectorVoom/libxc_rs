//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 877/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk877<F: Float>(t1971: F, t236: F, t5567: F, t8517: F, t2160: F, t638: F, t8850: F, t8854: F, t131: F, t4999: F, t639: F, t71: F, t5055: F, t7769: F, t1341: F, t575: F, t7310: F) -> (F, F, F, F, F, F) {
    let t42011 = t8517 * t1971 * t236 * t5567;
    let t42023 = t638 * t2160 * t8850;
    let t42026 = t638 * t2160 * t8854;
    let t42032 = t638 * t639 * t71 * t4999 * t131;
    let t42034 = t5055 * t7769;
    let t42042 = t638 * t7310 * t575 * t1341;
    (t42011, t42023, t42026, t42032, t42034, t42042)
}
