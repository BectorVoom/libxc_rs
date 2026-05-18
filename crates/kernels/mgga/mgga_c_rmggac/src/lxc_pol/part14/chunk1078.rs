//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1078/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1078<F: Float>(t36935: F, t9082: F, t202: F, t461: F, t5527: F, t674: F, t678: F, t2185: F, t9086: F, t16043: F, t9051: F, t9055: F) -> (F, F, F, F, F) {
    let t42250 = t36935 * t9082;
    let t42255 = t5527 * t202 * t461 * t674 * t678;
    let t42258 = t9086 * t2185 * t678;
    let t42259 = F::new(0.19863479950205658386e-4) * t42258;
    let t42260 = t16043 * t9051;
    let t42262 = t16043 * t9055;
    (t42250, t42255, t42259, t42260, t42262)
}
