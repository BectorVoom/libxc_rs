//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 805/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk805<F: Float>(t395: F, t4973: F, t16672: F, t16677: F, t16682: F, t16686: F, t16690: F, t16693: F, t16697: F, t16701: F, t16705: F, t16706: F, t1662: F, t1763: F, t16669: F, t11: F, t4949: F) -> (F, F, F, F, F) {
    let t16708 = t395 * t4973;
    let t16710 = 0.45340000000000000001e-1 * t16672 - 0.45340000000000000002e-1 * t16677 + 0.37783333333333333335e-2 * t16682 + 0.5037777777777777778e-2 * t16686 - 0.4534e-1 * t16690 + 0.6801e-1 * t16693 - 0.11335e-1 * t16697 - 0.15113333333333333333e-1 * t16701 - t16705 - 0.15113333333333333333e-1 * t16706 + 0.15113333333333333333e-1 * t16708;
    let t16712 = 1.0 / t1662 / t1763;
    let t16713 = t16712 * t16669;
    let t16715 = t11 * t4949 * t16713;
    (t16708, t16710, t16712, t16713, t16715)
}
