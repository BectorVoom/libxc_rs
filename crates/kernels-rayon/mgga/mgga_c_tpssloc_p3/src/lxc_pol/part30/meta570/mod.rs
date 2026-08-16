//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta570(t28550: f64, t28592: f64, t349: f64, t1945: f64, t5872: f64, t3201: f64, t1615: f64, t7593: f64, t1060: f64, t25523: f64, t7610: f64, t1539: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28593, t28594, t28596, t28597, t28601, t28602, t28605, t28609) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1940(t28550, t28592, t349, t1945, t5872, t3201, t1615, t7593, t1060, t25523, t7610, t1539, t25516);
    (t28593, t28594, t28596, t28597, t28601, t28602, t28605, t28609)
}
