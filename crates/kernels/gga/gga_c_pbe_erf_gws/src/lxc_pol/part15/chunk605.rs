//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 605/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk605<F: Float>(t2796: F, t561: F, t198: F, t34: F, t2735: F, t1046: F, t633: F, t1006: F, t583: F, t1689: F, t1743: F, t2696: F, t2699: F, t2702: F, t2707: F) -> (F, F, F, F, F, F, F) {
    let t2797 = t561 * t2796;
    let t2798 = F::new(8.0) / F::new(45.0) * t2797;
    let t2799 = t198 * t34;
    let t2800 = t2735 * t2799;
    let t2802 = F::new(4.0) / F::new(15.0) * t561 * t2800;
    let t2806 = F::new(2.0) / F::new(15.0) * t633 * t1046;
    let t2807 = t1006 * t583;
    let t2808 = F::new(4.0) / F::new(45.0) * t2807;
    let t2814 = -t1743 - F::cast_from(0.62972222222222222223e-3_f64) * t1689 - F::cast_from(0.62972222222222222223e-3_f64) * t2696 + F::cast_from(0.12594444444444444445e-2_f64) * t2699 - F::cast_from(0.37783333333333333334e-2_f64) * t2702 - F::cast_from(0.37783333333333333334e-2_f64) * t2707;
    (t2798, t2799, t2800, t2802, t2806, t2808, t2814)
}
