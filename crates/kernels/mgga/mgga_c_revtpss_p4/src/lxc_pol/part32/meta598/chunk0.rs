//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1932/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1932<F: Float>(t2110: F, t5808: F, t1455: F, t8130: F, t1921: F, t7541: F, t28944: F, t575: F, t5891: F, t94978: F, t665: F, t94982: F) -> (F, F, F, F, F, F) {
    let t104079 = F::new(2.0) * t2110 * t5808;
    let t104081 = F::new(2.0) * t1455 * t8130;
    let t104083 = F::new(2.0) * t7541 * t1921;
    let t104085 = F::new(2.0) * t28944 * t575;
    let t105870 = t94978 * t5891;
    let t105872 = t5891 * t665;
    let t105873 = t94982 * t105872;
    (t104079, t104081, t104083, t104085, t105870, t105873)
}
