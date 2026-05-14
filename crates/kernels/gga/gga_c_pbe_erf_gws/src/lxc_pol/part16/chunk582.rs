//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 582/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk582<F: Float>(t1023: F, t582: F, t616: F, t1018: F, t185: F, t1001: F, t395: F, t1758: F, t2561: F, t11: F, t2555: F, t571: F, t34: F, t572: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2753 = t582 * t1023;
    let t2754 = t616 * t2753;
    let t2755 = 8.0 / 45.0 * t2754;
    let t2756 = t582 * t1018;
    let t2757 = t185 * t2756;
    let t2758 = 4.0 / 45.0 * t2757;
    let t2760 = t395 * t1001;
    let t2762 = t1758 * t2561;
    let t2763 = t11 * t2762;
    let t2765 = t571 * t2555;
    let t2766 = t11 * t2765;
    let t2768 = t572 * t34;
    (t2753, t2755, t2756, t2758, t2760, t2762, t2763, t2765, t2766, t2768)
}
