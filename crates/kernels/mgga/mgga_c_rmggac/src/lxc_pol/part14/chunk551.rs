//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 551/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk551<F: Float>(t289: F, t7399: F, t1990: F, t2186: F, t1271: F, t1986: F, t675: F, t4443: F, t671: F, t674: F, t1175: F, t128: F) -> (F, F, F, F, F, F, F) {
    let t7400 = t289 * t7399;
    let t7401 = F::new(0.4726e1) * t7400;
    let t7402 = t2186 * t1990;
    let t7404 = t1986 * t1271;
    let t7405 = t675 * t7404;
    let t7406 = F::new(0.85129199786595678796e-5) * t7405;
    let t7407 = t671 * t4443;
    let t7408 = t7407 * t674;
    let t7409 = t128 * t1175;
    (t7401, t7402, t7404, t7406, t7407, t7408, t7409)
}
