//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta576(t1864: f64, t4021: f64, t1410: f64, t9231: f64, t2240: f64, t3961: f64, t3967: f64, t12571: f64, t608: f64, t645: f64, t7445: f64, t26351: f64, t6883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t90094, t90098, t90101, t90104, t90114, t90247, t90459) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1812(t1864, t4021, t1410, t9231, t2240, t3961, t3967, t12571, t608, t645, t7445, t26351, t6883);
    (t90094, t90098, t90101, t90104, t90114, t90247, t90459)
}
