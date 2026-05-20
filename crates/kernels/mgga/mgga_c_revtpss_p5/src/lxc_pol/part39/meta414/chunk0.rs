//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1492/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1492<F: Float>(t31377: F, t571: F, t1464: F, t8372: F, t2178: F, t2371: F, t670: F, t8273: F, t31027: F, t31271: F, t116929: F, t8358: F) -> (F, F, F, F, F, F) {
    let t117369 = F::new(2.0) * t571 * t31377;
    let t117374 = F::new(2.0) * t8372 * t1464;
    let t117381 = t2371 * t2178;
    let t117385 = t670 * t8273;
    let t117450 = F::new(4.0) / F::new(3.0) * t31027 * t31271;
    let t117457 = t116929 * t8358;
    (t117369, t117374, t117381, t117385, t117450, t117457)
}
