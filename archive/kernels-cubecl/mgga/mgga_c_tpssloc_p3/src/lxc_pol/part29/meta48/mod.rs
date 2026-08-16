//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta48 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk336;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk337;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk338;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk339;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta48<F: Float>(t880: F, t906: F, t886: F, t897: F, t902: F, t910: F, t310: F, t324: F, t320: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t926, t929, t931) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk336::<F>(t880, t906, t886, t897, t902, t910);
        let t932 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk337::<F>(t310);
        let (t933, t936, t938) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk338::<F>(t931, t932, t880, t886);
        let (t939, t941, t942) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk339::<F>(t324, t938, t320);
        let t943 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk340::<F>(t315, t942);
    (t926, t929, t931, t932, t933, t936, t938, t939, t941, t942, t943)
}
