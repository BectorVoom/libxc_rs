//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 699/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk699<F: Float>(t35620: F, t7558: F, t290: F, t35604: F, t2010: F, t7755: F, t1341: F, t303: F, t638: F, t7310: F, t357: F, t7254: F, t7364: F, t7243: F, t1973: F, t1965: F, t7942: F) -> (F, F, F, F, F, F, F, F, F) {
    let t35621 = t35620 * t7558;
    let t35623 = t290 * t35604;
    let t35625 = t2010 * t7755 * t35623;
    let t35629 = t638 * t7310 * t303 * t1341;
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    let t35654 = t7254 * t7243;
    let t35655 = t35654 * t1973;
    let t35657 = t7942 * t1965;
    (t35621, t35623, t35625, t35629, t35633, t35637, t35654, t35655, t35657)
}
