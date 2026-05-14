//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 529/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk529<F: Float>(t2790: F, t564: F, t1006: F, t612: F, t1883: F, t582: F, t996: F, t561: F, t198: F, t34: F, t2735: F, t1046: F, t633: F, t583: F, t1689: F, t1743: F, t2696: F, t2699: F, t2702: F, t2707: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2792 = 4.0 / 15.0 * t2790 * t564;
    let t2794 = 2.0 / 15.0 * t1006 * t612;
    let t2795 = 8.0 / 45.0 * t1883;
    let t2796 = t582 * t996;
    let t2797 = t561 * t2796;
    let t2798 = 8.0 / 45.0 * t2797;
    let t2799 = t198 * t34;
    let t2800 = t2735 * t2799;
    let t2802 = 4.0 / 15.0 * t561 * t2800;
    let t2806 = 2.0 / 15.0 * t633 * t1046;
    let t2807 = t1006 * t583;
    let t2808 = 4.0 / 45.0 * t2807;
    let t2814 = -t1743 - 0.62972222222222222223e-3 * t1689 - 0.62972222222222222223e-3 * t2696 + 0.12594444444444444445e-2 * t2699 - 0.37783333333333333334e-2 * t2702 - 0.37783333333333333334e-2 * t2707;
    (t2792, t2794, t2795, t2796, t2797, t2798, t2799, t2800, t2802, t2806, t2807, t2808, t2814)
}
