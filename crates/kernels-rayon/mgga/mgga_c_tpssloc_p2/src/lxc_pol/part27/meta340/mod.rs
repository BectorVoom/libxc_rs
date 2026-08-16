//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1421;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta340(t10027: f64, t541: f64, t12267: f64, t1362: f64, t3777: f64, t3865: f64, t1369: f64, t1361: f64, t2690: f64, t1336: f64, t241: f64, t67: f64, t6924: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12335, t12336, t12339, t12340, t12345, t12346, t12351) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1421(t10027, t541, t12267, t1362, t3777, t3865, t1369, t1361, t2690, t1336, t241, t67, t6924);
    (t12335, t12336, t12339, t12340, t12345, t12346, t12351)
}
