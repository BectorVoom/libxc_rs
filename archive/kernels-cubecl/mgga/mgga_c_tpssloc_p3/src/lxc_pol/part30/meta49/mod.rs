//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta49 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk339;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk340;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk341;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk342;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta49<F: Float>(t880: F, t906: F, t886: F, t897: F, t902: F, t910: F, t323: F, t300: F, t311: F, t890: F, t916: F, t919: F, t924: F, t933: F, t939: F, t943: F, t315: F, t942: F, t338: F, t615: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t945, t948, t950) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk339::<F>(t880, t906, t886, t897, t902, t910);
        let t951 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk340::<F>(t323);
        let t952 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk341::<F>(t950, t951);
        let (t956, t958, t959) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk342::<F>(t300, t311, t890, t916, t919, t924, t933, t939, t943, t952, t315);
        let (t961, t963, t964) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk343::<F>(t942, t950, t951, t959, t338, t615);
    (t945, t948, t950, t951, t952, t956, t958, t959, t961, t963, t964)
}
