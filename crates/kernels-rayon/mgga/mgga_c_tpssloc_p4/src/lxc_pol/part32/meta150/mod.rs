//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk804;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta150(t40: f64, t52: f64, t185: f64, t4098: f64, t1472: f64, t751: f64, t1409: f64, t707: f64, t75: f64, t3966: f64, t607: f64, t767: f64, t78: f64, t771: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4099, t4100, t4101, t4102, t4103, t4104, t4110, t4111, t4117) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk804(t40, t52, t185, t4098, t1472, t751, t1409, t707, t75, t3966, t607, t767, t78, t771, zeta_threshold);
        let t4119 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk805(t4110, t4117);
    (t4099, t4100, t4101, t4102, t4103, t4104, t4111, t4119)
}
