//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3223/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3223<F: Float>(t39766: F, t49926: F, t49929: F, t1544: F, t2408: F, t49940: F, t18569: F, t2398: F, t39774: F, t14397: F, t14436: F, t18875: F, t2403: F, t39760: F, t39764: F, t39770: F, t39773: F) -> (F, F, F, F, F, F, F) {
    let t61149 = F::new(4.0) * t39766;
    let t61150 = F::cast_from(0.43374325201206959368e-1_f64) * t49926;
    let t61151 = F::cast_from(0.43374325201206959368e-1_f64) * t49929;
    let t61155 = t1544 * t2408;
    let t61159 = F::cast_from(0.70178683471615754484e1_f64) * t49940;
    let t61161 = F::new(8.0) * t2398 * t18569;
    let t61162 = F::cast_from(0.5848223622634646207e0_f64) * t39774;
    let t61163 = -F::new(12.0) * t14397 * t18875 * t2403 + F::new(12.0) * t14436 * t2403 * t61155 + t39760 - t39764 + t39770 + t39773 + t61149 - t61150 + t61151 + t61159 + t61161 - t61162;
    (t61149, t61150, t61151, t61159, t61161, t61162, t61163)
}
