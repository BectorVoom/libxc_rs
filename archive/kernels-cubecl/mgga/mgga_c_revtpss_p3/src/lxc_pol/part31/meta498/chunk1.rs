//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1815/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1815<F: Float>(t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t72: F, t7307: F, t686: F, t7284: F, t1426: F, t786: F) -> (F, F, F, F, F, F, F, F) {
    let t26040 = F::cast_from(0.13009920719177044025e-1_f64) * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = F::cast_from(0.11565819519348392139e-2_f64) * t26041 * t3908;
    let t26049 = t7307 * t72;
    let t26050 = t26049 * t686;
    let t26051 = t7284 * t26050;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    (t26040, t26041, t26043, t26049, t26050, t26051, t26053, t26054)
}
