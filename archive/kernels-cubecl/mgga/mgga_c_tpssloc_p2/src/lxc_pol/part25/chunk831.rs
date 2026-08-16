//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 831/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk831<F: Float>(t10680: F, t10695: F, t913: F, t893: F, t2840: F, t891: F, t275: F, t2843: F, t290: F, t10662: F, t10524: F, t2929: F, t951: F) -> (F, F, F) {
    let t10696 = t10680 + t10695;
    let t10697 = t10696 * t913;
    let t10699 = F::cast_from(1.0_f64) * t893 * t10697;
    let t10701 = F::cast_from(1.0_f64) / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = F::cast_from(1.0_f64) / t2843 / t290;
    let t10705 = t10662 * t10704;
    let t10707 = F::cast_from(0.51726012919273400301e3_f64) * t10702 * t10705;
    let t10709 = t2929 * t10524 * t951;
    (t10699, t10707, t10709)
}
