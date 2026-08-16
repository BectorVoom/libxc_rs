//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk806;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta169(t135: f64, t1606: f64, t973: f64, t3966: f64, t998: f64, t974: f64, t1041: f64, t1607: f64, t1622: f64, t2960: f64, t3039: f64, t3048: f64, t3054: f64, t3070: f64, t3084: f64, t3092: f64, t3130: f64, t4562: f64, t4565: f64, t4572: f64, t4575: f64, t4579: f64, t4585: f64, t4590: f64, t4596: f64, t4600: f64, t225: f64, t4552: f64, t68: f64, t369: f64, t1031: f64, t1611: f64, t1036: f64, t1612: f64, t1616: f64, t248: f64, t3101: f64, t1020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4603, t4604, t4608, t4613) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk806(t135, t1606, t973, t3966, t998, t974, t1041, t1607, t1622, t2960, t3039, t3048, t3054, t3070, t3084, t3092, t3130, t4562, t4565, t4572, t4575, t4579, t4585, t4590, t4596, t4600);
        let (t4615, t4616, t4617, t4622, t4625, t4630, t4631) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk807(t225, t4552, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101, t1020);
    (t4603, t4604, t4608, t4613, t4615, t4616, t4617, t4622, t4625, t4630, t4631)
}
