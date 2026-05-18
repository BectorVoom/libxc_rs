//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 903/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk903<F: Float>(t10491: F, t871: F, t10695: F, t311: F, t309: F, t10051: F, t1160: F, t265: F, t42109: F, t2486: F, t2568: F, t676: F, t754: F) -> (F, F, F, F, F, F, F) {
    let t44528 = t10491 * t871;
    let t44600 = F::new(1.0) / t10695 / t311;
    let t44601 = t309 * t44600;
    let t51340 = t1160 * t10051;
    let t51669 = t42109 * t265;
    let t51687 = t2486 * t2568;
    let t51853 = t676 * t754;
    (t44528, t44600, t44601, t51340, t51669, t51687, t51853)
}
