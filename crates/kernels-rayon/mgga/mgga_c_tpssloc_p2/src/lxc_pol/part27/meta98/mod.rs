//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk640;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk641;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta98(t2022: f64, t3: f64, t1401: f64, t1873: f64, t577: f64, t11: f64, t2: f64, t584: f64, t16: f64, t9: f64, t587: f64, t591: f64, t14: f64, t21: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2023, t2029, t2218, t2219) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk640(t2022, t3, t1401, t1873, t577, t11, t2, t584);
        let (t2220, t2221) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk641(t2219, t16, t9);
        let (t2222, t2223, t2224, t2225) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk642(t2221, t587, t591, t14, t21);
    (t2023, t2029, t2218, t2219, t2220, t2221, t2222, t2223, t2224, t2225)
}
