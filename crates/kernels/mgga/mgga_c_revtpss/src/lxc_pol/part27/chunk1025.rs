//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1025/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1025<F: Float>(t1426: F, t2023: F, t786: F, t3917: F, t25953: F, t7284: F, t1445: F, t7242: F, t689: F, t7275: F, t1364: F, t26050: F, t7289: F, t25304: F, t7283: F, t25946: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26053 = t2023 * t1426;
    let t26054 = t786 * t26053;
    let t26055 = t26054 * t3917;
    let t26058 = 0.96373646535613327357e-2 * t7284 * t25953;
    let t26061 = t7242 * t1445;
    let t26062 = t689 * t26061;
    let t26064 = t786 * t7275;
    let t26065 = t26064 * t1364;
    let t26067 = t7289 * t26050;
    let t26069 = t25304 * t7283;
    let t26071 = 0.22849835011101738147e-2 * t26069 * t25946;
    (t26053, t26054, t26055, t26058, t26061, t26062, t26064, t26065, t26067, t26069, t26071)
}
