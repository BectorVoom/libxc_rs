//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 233/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk233<F: Float>(t45: F, t606: F, t766: F, t81: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t769 = piecewise3::<F>(t151, F::new(0.0), F::new(2.0) / F::new(3.0) * t766 * t606);
    let t770 = F::new(1.0) / t81;
    (t769, t770)
}
