//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk222;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk223;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta29(t576: f64, t577: f64, t11: f64, t2: f64, t10: f64, t3: f64, t9: f64, t16: f64, t15: f64, t14: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t580, t581, t582, t583, t584, t586, t587, t588) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk222(t576, t577, t11, t2, t10, t3, t9, t16);
        let (t589, t590, t591) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk223(t588, t15, t3);
        let t592 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk224(t14, t591);
    (t580, t581, t582, t583, t584, t586, t587, t588, t589, t590, t591, t592)
}
