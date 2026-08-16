//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta587(t3701: f64, t6995: f64, t1307: f64, t2018: f64, t7752: f64, t1458: f64, t576: f64, t2113: f64, t1390: f64, t22811: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31035, t31299, t33136, t33185, t33690, t34999, t39041, t39054) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1975(t3701, t6995, t1307, t2018, t7752, t1458, t576, t2113, t1390, t22811, t601, t9238);
    (t31035, t31299, t33136, t33185, t33690, t34999, t39041, t39054)
}
