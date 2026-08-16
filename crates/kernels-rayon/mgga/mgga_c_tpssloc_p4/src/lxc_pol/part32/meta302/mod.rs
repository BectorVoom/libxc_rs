//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta302(t1053: f64, t68: f64, t1887: f64, t337: f64, t615: f64, t134: f64, t976: f64, t984: f64, t271: f64, t2775: f64, t974: f64, t2769: f64, t632: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10165, t10186, t10189, t10190, t10213, t10214, t10216) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1327(t1053, t68, t1887, t337, t615, t134, t976, t984, t271, t2775, t974, t2769, t632);
    (t10165, t10186, t10189, t10190, t10213, t10214, t10216)
}
