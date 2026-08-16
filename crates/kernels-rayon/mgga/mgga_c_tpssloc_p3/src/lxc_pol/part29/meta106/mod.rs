//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta106 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk672;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk673;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta106(t138: f64, t681: f64, t125: f64, t2412: f64, t702: f64, t118: f64, t142: f64, t2393: f64, t706: f64, t717: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk672(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk673(t118, t142, t2393);
        let t2427 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk674(t706, t717);
    (t2419, t2420, t2421, t2423, t2426, t2427)
}
