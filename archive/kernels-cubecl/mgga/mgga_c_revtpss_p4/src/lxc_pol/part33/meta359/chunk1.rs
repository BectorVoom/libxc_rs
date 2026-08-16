//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1383/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1383<F: Float>(t14224: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t1892: F, t4086: F, t786: F, t4104: F) -> (F, F, F, F, F, F) {
    let t14225 = t4100 * t14224;
    let t14227 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14225;
    let t14229 = F::cast_from(0.19514881078765566038e-1_f64) * t10014 * t5741;
    let t14230 = t13790 * t1398;
    let t14231 = t10022 * t14230;
    let t14233 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t14231;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14241 = F::cast_from(0.19514881078765566038e-1_f64) * t14239 * t4104;
    (t14227, t14229, t14230, t14233, t14239, t14241)
}
