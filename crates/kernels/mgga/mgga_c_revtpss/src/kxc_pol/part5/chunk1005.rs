//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1005/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1005<F: Float>(t10981: F, t10982: F, t2455: F, t9285: F, t2454: F, t252: F, t2769: F, t786: F, t2435: F, t2448: F, t2440: F, t887: F) -> (F, F, F, F, F) {
    let t10984 = F::new(0.19637199382202157274e-3) * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = F::new(0.46263278077393568556e-2) * t2454 * t10985;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11000 = t2435 * t2448;
    let t11003 = t2440 * t887;
    (t10984, t10987, t10995, t11000, t11003)
}
