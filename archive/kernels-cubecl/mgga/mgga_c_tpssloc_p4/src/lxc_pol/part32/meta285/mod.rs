//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1290;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta285<F: Float>(t590: F, t60: F, t192: F, t533: F, t1390: F, t2018: F, t16: F, t2: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t8705, t8944, t8945, t9212, t9214, t9216) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1290::<F>(t590, t60, t192, t533, t1390, t2018, t16, t2, t591, t9, t21, t587);
        let (t9218, t9220, t9222, t9223) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1291::<F>(t14, t598, t2230, t594, t2229, t3);
    (t8705, t8944, t8945, t9212, t9214, t9216, t9218, t9220, t9222, t9223)
}
