//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk694;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk695;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta115<F: Float>(t252: F, t2591: F, t798: F, t852: F, t225: F, t799: F, t154: F, t2559: F, t222: F, t2563: F, t805: F, t119: F, t2379: F, t210: F, t2553: F, t237: F, t68: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2592, t2594, t2597) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk694::<F>(t252, t2591, t798, t852, t225, t799);
        let (t2600, t2602, t2603, t2606, t2610, t2613) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk695::<F>(t154, t2559, t222, t2563, t805, t119, t2379, t210, t2553, t225, t2591);
        let (t2614, t2617) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk696::<F>(t237, t2613, t68, t808);
    (t2592, t2594, t2597, t2600, t2602, t2603, t2606, t2610, t2613, t2614, t2617)
}
