//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1846;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta568(t1888: f64, t232: f64, t47448: f64, t6646: f64, t23110: f64, t23185: f64, t25241: f64, t25038: f64, t25248: f64, t25249: f64, t2553: f64, t1519: f64, t2631: f64, t1484: f64, t852: f64, t776: f64, t13393: f64, t22996: f64, t22986: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87097, t87100, t87104, t87106) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1846(t1888, t232, t47448, t6646, t23110, t23185, t25241, t25038, t25248, t25249, t2553, t1519, t2631);
        let (t87109, t87111, t87114, t87117, t87124) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1847(t1888, t232, t6646, t87106, t1484, t852, t25038, t25248, t776, t13393, t22996, t22986, t25249, t2633);
    (t87097, t87100, t87104, t87106, t87109, t87111, t87114, t87117, t87124)
}
