//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta534(t2148: f64, t4930: f64, t1716: f64, t7381: f64, t3502: f64, t491: f64, t24813: f64, t1011: f64, t1734: f64, t4978: f64, t1209: f64, t1216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27481, t27484, t27488, t27489, t27490, t27491, t27492, t27495, t27496, t27497) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1872(t2148, t4930, t1716, t7381, t3502, t491, t24813, t1011, t1734, t4978, t1209, t1216);
    (t27481, t27484, t27488, t27489, t27490, t27491, t27492, t27495, t27496, t27497)
}
