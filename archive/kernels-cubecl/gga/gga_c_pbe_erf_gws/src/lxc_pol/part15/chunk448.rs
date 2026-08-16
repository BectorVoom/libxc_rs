//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 448/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk448<F: Float>(t1730: F, t619: F, t1406: F, t220: F, t186: F, t616: F, t633: F, t663: F, t582: F, t611: F, t185: F, t1687: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1732 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1730 * t619;
    let t1733 = -t1406;
    let t1734 = t220 * t1733;
    let t1735 = t186 * t1734;
    let t1737 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t1735;
    let t1739 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t633 * t663;
    let t1740 = t582 * t611;
    let t1741 = t185 * t1740;
    let t1742 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1741;
    let t1743 = F::cast_from(0.25188888888888888889e-2_f64) * t1687;
    (t1732, t1733, t1734, t1735, t1737, t1739, t1740, t1741, t1742, t1743)
}
