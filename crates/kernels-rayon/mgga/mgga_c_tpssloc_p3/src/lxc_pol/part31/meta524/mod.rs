//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta524(t3701: f64, t7216: f64, t31: f64, t63: f64, t1458: f64, t576: f64, t2035: f64, t7939: f64, t1409: f64, t1390: f64, t22811: f64, t601: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t32193, t33185, t33234, t33899, t34125, t35259, t39041, t39054) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1738(t3701, t7216, t31, t63, t1458, t576, t2035, t7939, t1409, t1390, t22811, t601, t9238);
    (t32193, t33185, t33234, t33899, t34125, t35259, t39041, t39054)
}
