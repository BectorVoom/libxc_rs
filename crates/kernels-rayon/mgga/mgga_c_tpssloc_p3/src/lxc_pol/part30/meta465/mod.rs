//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta465(t210: f64, t6679: f64, t3139: f64, t6717: f64, t3113: f64, t6754: f64, t3107: f64, t6753: f64, t1012: f64, t1933: f64, t607: f64, t1937: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t23422, t23425, t23433, t23436, t23437, t23443) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1750(t210, t6679, t3139, t6717, t3113, t6754, t3107, t6753, t1012, t1933, t607, t1937);
    (t23422, t23425, t23433, t23436, t23437, t23443)
}
