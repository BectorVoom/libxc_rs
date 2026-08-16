//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1420;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1421;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta352(t1409: f64, t2250: f64, t65: f64, t3966: f64, t607: f64, t3961: f64, t628: f64, t12606: f64, t31: f64, t3967: f64, t2244: f64, t9287: f64, t2267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12648 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1420(t1409, t2250);
        let (t12649, t12652) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1421(t12648, t65, t3966, t607);
        let (t12653, t12656, t12661, t12662, t12665, t12677, t12680) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1422(t12652, t65, t3961, t628, t12606, t31, t3967, t1409, t2244, t9287, t2267, t3966);
    (t12648, t12649, t12652, t12653, t12656, t12661, t12662, t12665, t12677, t12680)
}
