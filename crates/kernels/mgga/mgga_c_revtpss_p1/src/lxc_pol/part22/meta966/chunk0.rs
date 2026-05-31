//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3229/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3229<F: Float>(t4343: F, t177: F, t18550: F, t762: F, t50092: F, t50094: F, t123: F, t2630: F, t5941: F, t50097: F, t50099: F, t14390: F, t18259: F) -> (F, F, F, F, F, F, F, F) {
    let t61234 = t4343 * t4343;
    let t61239 = t18550 * t177 * t762;
    let t61240 = F::cast_from(0.11696447245269292414e1_f64) * t61239;
    let t61244 = F::cast_from(0.32530743900905219526e-1_f64) * t50092;
    let t61245 = F::cast_from(0.96319466275353142155e0_f64) * t50094;
    let t61247 = t5941 * t123 * t2630;
    let t61248 = F::cast_from(0.10843581300301739842e-1_f64) * t61247;
    let t61249 = F::cast_from(4.0_f64) * t50097;
    let t61250 = F::cast_from(16.0_f64) * t50099;
    let t61261 = F::cast_from(48.0_f64) * t18259 * t14390;
    (t61234, t61240, t61244, t61245, t61248, t61249, t61250, t61261)
}
