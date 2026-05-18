//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1103/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1103<F: Float>(t40803: F, t40831: F, t118: F, t305: F, t326: F, t40806: F, t40809: F, t40814: F, t40824: F, t40827: F, t40834: F, t43080: F, t43644: F, t43749: F, t43971: F) -> F {
    let t44029 = F::new(0.3193131120497015617e0) * t40803;
    let t44035 = F::new(0.3193131120497015617e0) * t40831;
    let t44043 = -F::new(0.79828278012425390428e-1) * t118 * t43971 - t44029 - F::new(0.47896966807455234256e0) * t40806 - F::new(0.17961362552795712846e0) * t40809 - F::new(0.2993560425465952141e-1) * t40814 - F::new(0.35922725105591425692e0) * t40824 - F::new(0.11974241701863808564e0) * t40827 + t44035 - F::new(0.35922725105591425692e0) * t40834 + F::new(0.59871208509319042821e-1) * t305 * t43080 - F::new(0.11974241701863808564e0) * t326 * t43644 - F::new(0.59871208509319042821e-1) * t326 * t43749;
    t44043
}
