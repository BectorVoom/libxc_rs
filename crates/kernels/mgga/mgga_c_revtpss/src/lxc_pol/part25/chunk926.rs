//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 926/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk926<F: Float>(t10918: F, t10976: F, t868: F, t251: F, t9646: F, t22: F, t780: F, t2455: F, t9285: F, t2454: F, t2829: F, t779: F) -> (F, F, F, F, F, F, F) {
    let t10977 = t10918 + t10976;
    let t10978 = t868 * t10977;
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    let t10984 = F::new(0.19637199382202157274e-3) * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = F::new(0.46263278077393568556e-2) * t2454 * t10985;
    let t10988 = t779 * t2829;
    (t10977, t10978, t10982, t10984, t10985, t10987, t10988)
}
