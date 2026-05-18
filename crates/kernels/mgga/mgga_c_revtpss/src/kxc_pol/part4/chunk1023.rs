//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1023/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1023<F: Float>(t10981: F, t10982: F, t2455: F, t9285: F, t2454: F, t2829: F, t779: F, t689: F, t2444: F, t887: F, t252: F, t2769: F) -> (F, F, F, F, F) {
    let t10984 = F::new(0.19637199382202157274e-3) * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = F::new(0.46263278077393568556e-2) * t2454 * t10985;
    let t10988 = t779 * t2829;
    let t10989 = t689 * t10988;
    let t10991 = t2444 * t887;
    let t10992 = t689 * t10991;
    let t10994 = t252 * t2769;
    (t10984, t10987, t10989, t10992, t10994)
}
