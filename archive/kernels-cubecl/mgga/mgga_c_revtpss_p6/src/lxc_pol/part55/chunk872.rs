//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 872/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk872<F: Float>(t2470: F, t7514: F, t7284: F, t25878: F, t26234: F, t1445: F, t7492: F, t689: F, t1385: F, t2097: F, t7289: F, t25969: F) -> (F, F, F, F, F, F) {
    let t26292 = t7514 * t2470;
    let t26294 = F::cast_from(0.96373646535613327357e-2_f64) * t7284 * t26292;
    let t26295 = t25878 * t26234;
    let t26301 = t7492 * t1445;
    let t26302 = t689 * t26301;
    let t26304 = t1385 * t2097;
    let t26309 = F::cast_from(0.17135234354032049604e-1_f64) * t7289 * t26292;
    let t26310 = F::cast_from(0.54208002996571016773e-3_f64) * t25969;
    (t26294, t26295, t26302, t26304, t26309, t26310)
}
