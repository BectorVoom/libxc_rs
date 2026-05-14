//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 730/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk730<F: Float>(t2023: F, t2453: F, t3908: F, t72: F, t7307: F, t686: F, t7284: F, t1426: F, t786: F, t3917: F, t25953: F, t1445: F, t7242: F, t689: F, t7275: F, t1364: F) -> (F, F, F, F, F, F, F, F) {
    let t26041 = t2453 * t2023;
    let t26043 = 0.11565819519348392139e-2 * t26041 * t3908;
    let t26049 = t7307 * t72;
    let t26050 = t26049 * t686;
    let t26051 = t7284 * t26050;
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    let t26055 = t26054 * t3917;
    let t26058 = 0.96373646535613327357e-2 * t7284 * t25953;
    let t26061 = t7242 * t1445;
    let t26062 = t689 * t26061;
    let t26064 = t786 * t7275;
    let t26065 = t26064 * t1364;
    (t26043, t26050, t26051, t26054, t26055, t26058, t26062, t26065)
}
