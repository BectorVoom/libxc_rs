//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta46 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk315;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk316;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk317;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk318;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk319;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta46<F: Float>(t275: F, t892: F, t276: F, t880: F, t886: F, t273: F, t241: F, t697: F, t281: F, t283: F, t340: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t893 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk315::<F>(t275, t892);
        let t894 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk316::<F>(t276);
        let t896 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk317::<F>(t880, t886);
        let (t897, t899, t901) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk318::<F>(t894, t896, t880, t273);
        let (t902, t904, t906) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk319::<F>(t896, t901, t241, t697, t281, t283);
        let (t907, t908) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk320::<F>(t906, t241, t340);
    (t893, t894, t896, t897, t899, t901, t902, t904, t906, t907, t908)
}
