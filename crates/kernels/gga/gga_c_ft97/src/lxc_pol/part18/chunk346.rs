//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 346/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk346<F: Float>(t2086: F, t2087: F, t91: F, t151: F, t1771: F, t1775: F, t583: F, t458: F, t588: F, t143: F, t1554: F) -> (F, F, F, F, F) {
    let t2089 = t91 * t2086 * t2087;
    let t2092 = 4.0 / 9.0 * t1771 * t151;
    let t2093 = t1775 * t583;
    let t2095 = t458 * t588;
    let t2097 = t1554 * t143;
    (t2089, t2092, t2093, t2095, t2097)
}
