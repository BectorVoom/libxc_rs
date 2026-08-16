//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 842/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk842<F: Float>(t25304: F, t7057: F, t25301: F, t1032: F, t860: F, t867: F, t786: F, t7060: F, t11007: F, t233: F, t213: F, t7048: F) -> (F, F, F, F, F, F) {
    let t25305 = t25304 * t7057;
    let t25307 = F::cast_from(0.22849835011101738147e-2_f64) * t25305 * t25301;
    let t25308 = t860 * t1032;
    let t25309 = t25308 * t867;
    let t25310 = t786 * t25309;
    let t25311 = t25310 * t7060;
    let t25317 = t11007 * t233;
    let t25322 = t213 * t7048;
    (t25307, t25308, t25309, t25311, t25317, t25322)
}
