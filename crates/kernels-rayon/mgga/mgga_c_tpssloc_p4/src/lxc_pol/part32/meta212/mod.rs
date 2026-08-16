//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta212(t5685: f64, t882: f64, t123: f64, t2765: f64, t4335: f64, t5679: f64, t5683: f64, t291: f64, t1557: f64, t4354: f64, t1556: f64, t913: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5686, t5687, t5689, t5691, t5693, t5694, t5695) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1019(t5685, t882, t123, t2765, t4335, t5679, t5683, t291, t1557, t4354, t1556, t913);
    (t5686, t5687, t5689, t5691, t5693, t5694, t5695)
}
