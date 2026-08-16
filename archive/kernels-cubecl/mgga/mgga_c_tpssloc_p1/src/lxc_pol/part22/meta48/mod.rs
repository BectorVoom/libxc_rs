//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta48 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk342;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk343;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk344;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk345;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta48<F: Float>(t300: F, t311: F, t890: F, t916: F, t919: F, t924: F, t933: F, t939: F, t943: F, t952: F, t315: F, t942: F, t950: F, t951: F, t338: F, t615: F, t134: F, t340: F, t344: F, t221: F, t339: F, t209: F, t39: F, t119: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t956, t958, t959) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk342::<F>(t300, t311, t890, t916, t919, t924, t933, t939, t943, t952, t315);
        let (t961, t963, t964) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk343::<F>(t942, t950, t951, t959, t338, t615);
        let (t967, t969, t971, t972) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk344::<F>(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk345::<F>(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk346::<F>(t119, t60);
    (t956, t958, t959, t961, t963, t964, t967, t969, t971, t972, t973, t974)
}
