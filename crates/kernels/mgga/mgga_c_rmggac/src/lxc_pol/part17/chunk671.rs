//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 671/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk671<F: Float>(t9229: F, t2347: F, t570: F, t262: F, t7204: F, t558: F) -> (F, F, F, F, F) {
    let t9672 = F::new(0.5987120850931904282e-1) * t9229;
    let t9704 = t2347 * t570;
    let t9705 = t262 * t9704;
    let t9706 = t7204 * t9705;
    let t9707 = F::new(0.20455996240684006296e-1) * t9706;
    let t9708 = t2347 * t558;
    (t9672, t9704, t9705, t9707, t9708)
}
