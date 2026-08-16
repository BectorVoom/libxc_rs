//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1359;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1360;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta311<F: Float>(t2777: F, t690: F, t2781: F, t154: F, t3061: F, t10544: F, t276: F, t285: F, t273: F, t2897: F, t300: F, t2928: F, t941: F, t2931: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t10560 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1359::<F>(t2777, t690);
        let t10562 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1360::<F>(t2781, t690);
        let (t10564, t10577, t10595, t10599, t10608, t10623, t10629, t10632) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1361::<F>(t154, t3061, t10544, t276, t285, t273, t2897, t300, t2928, t941, t2931, t323);
    (t10560, t10562, t10564, t10577, t10595, t10599, t10608, t10623, t10629, t10632)
}
