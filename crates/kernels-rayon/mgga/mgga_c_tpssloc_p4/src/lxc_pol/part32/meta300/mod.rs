//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta300(t2617: f64, t2696: f64, t2693: f64, t809: f64, t597: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64, t154: f64, t9569: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t9993, t10014, t10022, t10024, t10026, t10027) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1325(t2617, t2696, t2693, t809, t597, t61, t241, t244, t248, t238, t154, t9569);
    (t9993, t10014, t10022, t10024, t10026, t10027)
}
