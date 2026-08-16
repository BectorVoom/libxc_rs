//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1914;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta644(t22986: f64, t25054: f64, t86873: f64, t6552: f64, t6555: f64, t98133: f64, t1880: f64, t25216: f64, t25224: f64, t25038: f64, t25040: f64, t28267: f64, t81651: f64, t82074: f64, t1888: f64, t23270: f64, t25044: f64, t4300: f64, t5527: f64, t857: f64, t865: f64, t23035: f64, t23237: f64, t28298: f64, t23204: f64, t81640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98196, t98199, t98202, t98205, t98213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1914(t22986, t25054, t86873, t6552, t6555, t98133, t1880, t25216, t25224, t25038, t25040, t28267, t81651, t82074);
        let (t98222, t98227, t98234, t98237) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1915(t1888, t23270, t25044, t4300, t5527, t857, t25038, t865, t23035, t23237, t28298, t23204, t81640);
    (t98196, t98199, t98202, t98205, t98213, t98222, t98227, t98234, t98237)
}
