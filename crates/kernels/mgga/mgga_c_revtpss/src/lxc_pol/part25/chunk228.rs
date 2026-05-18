//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 228/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk228<F: Float>(t45: F, t57: F, t760: F, t762: F, t206: F, t262: F, t78: F, t606: F, t81: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t764 = F::new(0.5848223622634646207e0) * t760 * t762;
    let t765 = t206 * t262;
    let t766 = F::new(1.0) / t78;
    let t769 = piecewise3::<f64>(t151, F::new(0.0), F::new(2.0) / F::new(3.0) * t766 * t606);
    let t770 = F::new(1.0) / t81;
    let t773 = piecewise3::<f64>(t155, F::new(0.0), -F::new(2.0) / F::new(3.0) * t770 * t606);
    let t775 = t769 / F::new(2.0) + t773 / F::new(2.0);
    (t764, t765, t766, t770, t775)
}
