//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1336/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1336<F: Float>(t11865: F, t3090: F, t3316: F, t994: F, t4891: F, t1016: F, t697: F, t1011: F, t11132: F, t126: F, t373: F, t828: F) -> (F, F, F, F, F, F) {
    let t11866 = t11865 * t3090;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11880 = t697 * t1016;
    let t11881 = t1011 * t11880;
    let t11890 = F::cast_from(0.25925925925925925926e-1_f64) * t11132;
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    (t11866, t11875, t11881, t11890, t11921, t11922)
}
