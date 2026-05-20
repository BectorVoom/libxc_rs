//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1999/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1999<F: Float>(t11874: F, t27492: F, t11988: F, t7132: F, t11997: F, t25503: F, t3141: F, t1052: F, t3089: F, t1087: F, t11970: F, t1973: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t93548 = t11874 * t27492;
    let t93555 = t7132 * t11988;
    let t93567 = t3141 * t25503 * t11997;
    let t93595 = sigma0 * t1052;
    let t93596 = t93595 * t3089;
    let t93597 = t1087 * t93596;
    let t93611 = F::cast_from(0.1270341277572436651e-3_f64) * t1973 * t11970;
    (t93548, t93555, t93567, t93596, t93597, t93611)
}
