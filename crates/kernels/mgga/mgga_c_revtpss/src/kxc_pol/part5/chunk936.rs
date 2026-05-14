//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 936/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk936<F: Float>(t231: F, t281: F, t68: F, t836: F, t10535: F, t2783: F, t860: F, t786: F, t760: F, t9323: F, t9318: F, t2609: F, t717: F, t162: F, t9544: F, t158: F) -> (F, F, F, F, F, F) {
    let t10538 = t281 * t68 * t836 * t231;
    let t10539 = t10535 * t10538;
    let t10541 = t2783 * t860;
    let t10542 = t786 * t10541;
    let t10552 = 0.51947577317044391277e2 * t760 * t9323;
    let t10554 = 0.35089341735807877242e1 * t760 * t9318;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    let t10566 = t158 * t10565;
    (t10539, t10542, t10552, t10554, t10563, t10566)
}
