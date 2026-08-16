//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk866;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk867;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta130(t1437: f64, t645: f64, t1409: f64, t607: f64, t25: f64, t28: f64, t65: f64, t2219: f64, zeta_threshold: f64, t31: f64, t1410: f64, t628: f64, t1426: f64, t608: f64, t2267: f64, t43: f64, t2274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3958, t3961) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk866(t1437, t645, t1409, t607);
        let (t3962, t3966) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk867(t25, t28, t3961, t65, t2219, zeta_threshold);
        let (t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk868(t31, t3966, t65, t1410, t628, t1426, t608, t1409, t2267, t607, t43, t2274);
    (t3958, t3961, t3962, t3966, t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990)
}
