//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 991/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk991<F: Float>(t10535: F, t10538: F, t2783: F, t860: F, t786: F, t760: F, t9323: F, t9318: F, t2609: F, t717: F, t162: F, t9544: F) -> (F, F, F, F, F, F) {
    let t10539 = t10535 * t10538;
    let t10541 = t2783 * t860;
    let t10542 = t786 * t10541;
    let t10552 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t9323;
    let t10554 = F::cast_from(0.35089341735807877242e1_f64) * t760 * t9318;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    (t10539, t10542, t10552, t10554, t10563, t10565)
}
