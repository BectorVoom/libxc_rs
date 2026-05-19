//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 694/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk694<F: Float>(t7250: F, t1941: F, t540: F, t1372: F, t546: F, t550: F, t7028: F, t807: F, t2018: F, t786: F, t1381: F, t1385: F, t64: F) -> (F, F, F, F, F, F, F, F) {
    let t7251 = F::new(7.0) / F::new(288.0) * t7250;
    let t7252 = t1941 * t540;
    let t7253 = t7252 * t1372;
    let t7256 = t546 * t7028 * t550;
    let t7257 = t807 * t7256;
    let t7258 = F::cast_from(0.14291339372689912324e-4_f64) * t7257;
    let t7259 = t786 * t2018;
    let t7260 = t7259 * t1381;
    let t7261 = F::cast_from(0.25410001404642664113e-4_f64) * t7260;
    let t7262 = t1385 * t64;
    (t7251, t7252, t7253, t7256, t7258, t7259, t7261, t7262)
}
