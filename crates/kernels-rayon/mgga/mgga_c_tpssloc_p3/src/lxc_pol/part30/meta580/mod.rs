//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta580(t3701: f64, t6995: f64, t1307: f64, t2018: f64, t1862: f64, t31: f64, t1458: f64, t1868: f64, t7752: f64, t576: f64, t1409: f64, t1390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t31035, t31299, t33085, t33136, t33185, t33567, t34999) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1959(t3701, t6995, t1307, t2018, t1862, t31, t1458, t1868, t7752, t576, t1409, t1390);
    (t31035, t31299, t33085, t33136, t33185, t33567, t34999)
}
