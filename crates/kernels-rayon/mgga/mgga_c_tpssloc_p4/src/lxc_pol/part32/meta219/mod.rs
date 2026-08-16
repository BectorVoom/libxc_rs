//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta219(t5828: f64, t977: f64, t3003: f64, t4384: f64, t5718: f64, t5721: f64, t5724: f64, t340: f64, t343: f64, t974: f64, t1597: f64, t2969: f64, t2986: f64, t4507: f64, t4529: f64, t5818: f64, t5821: f64, t5825: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5829, t5836, t5838, t5839, t5842, t5844, t5845, t5848) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1029(t5828, t977, t3003, t4384, t5718, t5721, t5724, t340, t343, t974, t1597, t2969, t2986, t4507, t4529, t5818, t5821, t5825, t973);
    (t5829, t5836, t5838, t5839, t5842, t5844, t5845, t5848)
}
