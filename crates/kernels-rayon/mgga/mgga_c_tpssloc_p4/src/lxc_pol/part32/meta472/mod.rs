//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1769;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta472(t24826: f64, t7378: f64, t7319: f64, t7327: f64, t1170: f64, t7381: f64, t2121: f64, t210: f64, t7371: f64, t7284: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24827, t24833, t24844, t24845, t24847) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1769(t24826, t7378, t7319, t7327, t1170, t7381, t2121, t210, t7371);
        let (t24848, t24849) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1770(t7284, t974, t24847);
    (t24827, t24833, t24844, t24845, t24847, t24848, t24849)
}
